use zed_extension_api::{
    register_extension, serde_json, settings::LspSettings, Command, Extension, LanguageServerId,
    Result, Worktree,
};

const SERVER_ID: &str = "pandocmd";
const SERVER_BINARY: &str = "pandocmd-lsp";

struct PandocMarkdownExtension;

impl Extension for PandocMarkdownExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let binary = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|settings| settings.binary);

        let command = binary
            .as_ref()
            .and_then(|binary| binary.path.clone())
            .or_else(|| worktree.which(SERVER_BINARY))
            .ok_or_else(|| {
                format!(
                    "{SERVER_BINARY} must be installed and available in $PATH, or configured via lsp.{SERVER_ID}.binary.path"
                )
            })?;

        let args = binary
            .as_ref()
            .and_then(|binary| binary.arguments.clone())
            .unwrap_or_default();

        let mut env = worktree.shell_env();
        if let Some(binary_env) = binary.and_then(|binary| binary.env) {
            env.extend(binary_env);
        }

        Ok(Command { command, args, env })
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|settings| settings.initialization_options)
            .unwrap_or_default();
        Ok(Some(settings))
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|settings| settings.settings)
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

register_extension!(PandocMarkdownExtension);
