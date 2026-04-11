# Pandoc Markdown for Zed

Pandoc-flavored Markdown support for Zed, backed by Tree-sitter.

This extension is intended to replace the default Markdown language after installation. It associates `.md`, `.markdown`, `.qmd`, and `.rmd` files with `Pandoc Markdown`, adds Tree-sitter-based syntax highlighting, and uses section-aware parsing for folding and outline structure.

## Scope

- Pandoc Markdown parsing via `tree-sitter-pandoc-markdown`
- Tree-sitter syntax highlighting for block and inline constructs
- Code fence, HTML, YAML, TOML, and LaTeX injections
- LaTeX math spans for `$...$`, `$$...$$`, `\\(...\\)`, and `\\[...\\]`
- Section outline support for heading-based navigation and folding
- No preview support

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
- This extension does not ship a language server.
