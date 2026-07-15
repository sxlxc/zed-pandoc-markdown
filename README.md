# Pandoc Markdown for Zed

Pandoc-flavored Markdown support for Zed, backed by Tree-sitter and the
`pandocmd-lsp` language server.

This extension is intended to replace the default Markdown language after installation. It associates `.md`, `.markdown`, `.qmd`, and `.rmd` files with `Pandoc Markdown`, adds Tree-sitter-based syntax highlighting, and uses section-aware parsing for folding and outline structure.

## Scope

- Pandoc Markdown parsing via `tree-sitter-pandoc-markdown`
- Tree-sitter syntax highlighting for block and inline constructs
- Code fence, HTML, YAML, TOML, and LaTeX injections
- LaTeX math spans for `$...$`, `$$...$$`, `\\(...\\)`, and `\\[...\\]`
- Section outline support for heading-based navigation and folding
- Language-server support for document symbols, hover, completion, definitions,
  references, and diagnostics
- Optional Pandoc CLI validation when `pandoc` is installed
- Theorem-family fenced-div snippets
- No preview support

## Formatting

Zed language extensions cannot set a default external formatter. To use Pandoc
through `editor: format` or format-on-save, install `pandoc` on your `PATH` and
add this to your Zed settings:

```json
{
  "languages": {
    "Pandoc Markdown": {
      "formatter": {
        "external": {
          "command": "pandoc",
          "arguments": [
            "-f",
            "markdown",
            "-t",
            "markdown",
            "--standalone"
          ]
        }
      },
      "format_on_save": "off"
    }
  }
}
```

The settings version intentionally omits the input and output paths: Zed sends
the focused buffer to an external formatter on standard input and replaces it
with standard output. Passing `-o {buffer_path}` directly would produce no
standard output and could replace the editor buffer with an empty document.
Change `format_on_save` to `"on"` only after checking that Pandoc's
whole-document rewrite preserves the constructs used by your documents.

## Snippets

The extension provides fenced-div snippets for theorem (`thm`), lemma (`lem`),
proposition (`prop`), corollary (`cor`), conjecture (`conj`), definition
(`def`), example (`ex`), exercise (`exr`), problem (`prob`), remark (`rem`),
and proof (`proof`). For example, `thm` expands to:

```markdown
::: {#thm:label .theorem}
Statement.
:::
```

The identifier and body are tab stops. Proofs omit an identifier and use only
the `.proof` class.

## Language Server

The extension downloads a matching `pandocmd-lsp` binary from the latest
`sxlxc/pandocmd-languageserver` GitHub release on first use. Supported release
assets are the platform-specific `pandocmd-lsp-*` archives for macOS, Linux,
and Windows on `aarch64` and `x86_64`.

You can still install `pandocmd-lsp` somewhere on your `PATH`:

```bash
cargo install --git git@github.com:sxlxc/pandocmd-languageserver.git --locked pandocmd-lsp
```

Or point Zed at a local build:

```json
{
  "lsp": {
    "pandocmd": {
      "binary": {
        "path": "/path/to/pandocmd-lsp",
        "arguments": []
      }
    }
  }
}
```

The language server is started over stdio for `Pandoc Markdown` buffers.

## Local Development

1. Open Zed.
2. Open the Extensions panel.
3. Choose `Install Dev Extension`.
4. Select this repository directory.

Zed will compile the pinned Tree-sitter grammars from the forked grammar repository:

- `pandoc_markdown`
- `pandoc_markdown_inline`

Zed writes generated grammar checkouts under `grammars/`. That directory is build state, not extension source.

## Troubleshooting

- If Zed reports that `grammars/pandoc_markdown` or `grammars/pandoc_markdown_inline` "already exists, but is not a git clone", the generated grammar checkout is stale.
- This happens after changing the grammar repository URL.
- Remove `grammars/` and install the dev extension again.

## Notes

- The hidden inline language exists only to support inline injections and highlighting.
- `+++` front matter is injected as TOML.
- `---` front matter is injected as YAML.
- `\\(...\\)` and `\\[...\\]` are tokenized as LaTeX math spans and injected into the LaTeX grammar.
- The grammar source of truth is `https://github.com/sxlxc/tree-sitter-pandoc-markdown` pinned by commit SHA in `extension.toml`.
- The language server binary is provided by `git@github.com:sxlxc/pandocmd-languageserver.git`.
