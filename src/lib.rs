use std::fs;

use zed::settings::LspSettings;
use zed_extension_api::{self as zed, serde_json, LanguageServerId, Result};

const SERVER_ID: &str = "pandocmd";
const SERVER_BINARY: &str = "pandocmd-lsp";
const SERVER_REPOSITORY: &str = "sxlxc/pandocmd-languageserver";
const SERVER_DOWNLOAD_PREFIX: &str = "pandocmd-lsp-";

type ReleaseAssetTarget = (
    &'static str,
    &'static str,
    zed::DownloadedFileType,
    &'static str,
);

struct PandocMarkdownExtension {
    cached_server_path: Option<String>,
}

impl PandocMarkdownExtension {
    fn server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = worktree.which(SERVER_BINARY) {
            return Ok(path);
        }

        if let Some(path) = &self.cached_server_path {
            if fs::metadata(path).map_or(false, |metadata| metadata.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            SERVER_REPOSITORY,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )
        .map_err(|err| {
            Self::installation_failed(
                language_server_id,
                format!("failed to fetch latest {SERVER_BINARY} release: {err}"),
            )
        })?;

        let (target, archive_extension, file_type, executable_name) = Self::release_asset_target()
            .map_err(|err| Self::installation_failed(language_server_id, err))?;
        let asset_name = format!("{SERVER_BINARY}-{target}.{archive_extension}");
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| {
                Self::installation_failed(
                    language_server_id,
                    format!("no {SERVER_BINARY} release asset found matching {asset_name}"),
                )
            })?;

        let version_dir = format!("{SERVER_BINARY}-{}", release.version);
        let extracted_dir = format!("{version_dir}/{SERVER_BINARY}-{target}");
        let binary_path = format!("{extracted_dir}/{executable_name}");

        if !fs::metadata(&binary_path).map_or(false, |metadata| metadata.is_file()) {
            fs::create_dir_all(&version_dir).map_err(|err| {
                Self::installation_failed(
                    language_server_id,
                    format!("failed to create {SERVER_BINARY} install directory: {err}"),
                )
            })?;

            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(&asset.download_url, &version_dir, file_type).map_err(|err| {
                Self::installation_failed(
                    language_server_id,
                    format!("failed to download {asset_name}: {err}"),
                )
            })?;

            if executable_name == SERVER_BINARY {
                zed::make_file_executable(&binary_path).map_err(|err| {
                    Self::installation_failed(
                        language_server_id,
                        format!("failed to make {SERVER_BINARY} executable: {err}"),
                    )
                })?;
            }

            Self::remove_old_server_versions(&version_dir);
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::None,
        );
        self.cached_server_path = Some(binary_path.clone());
        Ok(binary_path)
    }

    fn release_asset_target() -> Result<ReleaseAssetTarget> {
        match zed::current_platform() {
            (zed::Os::Mac, zed::Architecture::Aarch64) => Ok((
                "aarch64-apple-darwin",
                "tar.gz",
                zed::DownloadedFileType::GzipTar,
                SERVER_BINARY,
            )),
            (zed::Os::Mac, zed::Architecture::X8664) => Ok((
                "x86_64-apple-darwin",
                "tar.gz",
                zed::DownloadedFileType::GzipTar,
                SERVER_BINARY,
            )),
            (zed::Os::Linux, zed::Architecture::Aarch64) => Ok((
                "aarch64-unknown-linux-gnu",
                "tar.gz",
                zed::DownloadedFileType::GzipTar,
                SERVER_BINARY,
            )),
            (zed::Os::Linux, zed::Architecture::X8664) => Ok((
                "x86_64-unknown-linux-gnu",
                "tar.gz",
                zed::DownloadedFileType::GzipTar,
                SERVER_BINARY,
            )),
            (zed::Os::Windows, zed::Architecture::Aarch64) => Ok((
                "aarch64-pc-windows-msvc",
                "zip",
                zed::DownloadedFileType::Zip,
                "pandocmd-lsp.exe",
            )),
            (zed::Os::Windows, zed::Architecture::X8664) => Ok((
                "x86_64-pc-windows-msvc",
                "zip",
                zed::DownloadedFileType::Zip,
                "pandocmd-lsp.exe",
            )),
            _ => Err(format!(
                "{SERVER_BINARY} releases are available for macOS, Linux, and Windows on aarch64 or x86_64"
            )),
        }
    }

    fn remove_old_server_versions(current_version_dir: &str) {
        let Ok(entries) = fs::read_dir(".") else {
            return;
        };

        for entry in entries.flatten() {
            let file_name = entry.file_name();
            let Some(file_name) = file_name.to_str() else {
                continue;
            };

            if file_name != current_version_dir && file_name.starts_with(SERVER_DOWNLOAD_PREFIX) {
                let path = entry.path();
                if path.is_dir() {
                    let _ = fs::remove_dir_all(path);
                } else {
                    let _ = fs::remove_file(path);
                }
            }
        }
    }

    fn installation_failed(language_server_id: &LanguageServerId, message: String) -> String {
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Failed(message.clone()),
        );
        message
    }
}

impl zed::Extension for PandocMarkdownExtension {
    fn new() -> Self {
        Self {
            cached_server_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary = LspSettings::for_worktree(SERVER_ID, worktree)
            .ok()
            .and_then(|settings| settings.binary);

        let command = binary
            .as_ref()
            .and_then(|binary| binary.path.clone())
            .map(Ok)
            .unwrap_or_else(|| self.server_binary_path(language_server_id, worktree))?;

        let args = binary
            .as_ref()
            .and_then(|binary| binary.arguments.clone())
            .unwrap_or_default();

        let mut env = worktree.shell_env();
        if let Some(binary_env) = binary.and_then(|binary| binary.env) {
            env.extend(binary_env);
        }

        Ok(zed::Command { command, args, env })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree(SERVER_ID, worktree)
            .ok()
            .and_then(|settings| settings.initialization_options)
            .unwrap_or_default();
        Ok(Some(settings))
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree(SERVER_ID, worktree)
            .ok()
            .and_then(|settings| settings.settings)
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

zed::register_extension!(PandocMarkdownExtension);
