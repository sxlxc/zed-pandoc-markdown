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
- No preview support

## Language Server

Install `pandocmd-lsp` somewhere on your `PATH`:

```bash
cargo install --git git@github.com:sxlxc/pandocmd-languageserver.git --locked pandocmd-lsp
```

Alternatively, point Zed at a local build:

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
