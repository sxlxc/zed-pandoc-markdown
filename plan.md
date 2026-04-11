# Zed Pandoc Markdown Extension Plan

## Objective

Build a Zed language extension for Pandoc-flavored Markdown with:

- syntax-aware folding driven by Tree-sitter parsing
- strong Tree-sitter-based syntax highlighting
- correct inline/code-fence injections
- practical support for Pandoc-heavy files such as `md`, `qmd`, and `rmd`
- explicit replacement of Zed's default Markdown language after installation
- no preview functionality

## Grounding

- Zed language extensions can be purely declarative. Rust/WASM is only needed for procedural behavior, so this should be a language-only extension unless a later limitation forces custom code.
- Zed expects language assets under `languages/<language>/`, with `config.toml` plus query files such as `highlights.scm`, `injections.scm`, and `outline.scm`.
- Grammars are registered in `extension.toml` and pinned by `rev`. During local development, Zed can also load a grammar from a `file://` repository path.
- The upstream Pandoc grammar already exposes two Tree-sitter grammars:
  - `pandoc_markdown`
  - `pandoc_markdown_inline`
- The upstream repository already ships generated parser/scanner sources, so it is plausible to consume it directly for a first prototype.
- The upstream highlight queries are not directly Zed-ready. They use Neovim-style captures such as `@text.title`, `@text.uri`, `@text.reference`, `@text.emphasis`, and `@text.strong`, while Zed expects captures such as `@title`, `@link_uri`, `@link_text`, `@emphasis`, and `@emphasis.strong`.

## Scope Decisions

- MVP should stay focused on parsing, highlighting, injections, and folding-related structure.
- This extension is intended to replace the default Markdown language support, not coexist as a niche alternative.
- Do not add a language server in the first pass.
- Do not add Rust extension code in the first pass.
- Do not add preview support in the first pass or later unless the goals change.
- Treat the current empty `Cargo.toml` and empty `src/` as scaffolding that can be removed or left unused. They are not required for the MVP.
- Register `md`, `qmd`, and `rmd` to the Pandoc Markdown language by default.
- Treat compatibility with ordinary Markdown documents as a core requirement, because installing the extension should make standard `.md` files open as Pandoc Markdown.

## Proposed File Layout

```text
extension.toml
plan.md
README.md
LICENSE
languages/
  pandoc-markdown/
    config.toml
    highlights.scm
    injections.scm
    outline.scm
    textobjects.scm        # optional but useful
```

## Implementation Plan

### Phase 1: Clean the Scaffold

- Decide whether to keep or remove empty Rust/WASM files.
- Update `extension.toml` so it is a valid publishable manifest:
  - real `repository`
  - grammar registrations for `pandoc_markdown` and `pandoc_markdown_inline`
  - pinned upstream commit SHA
- Add `LICENSE`, because published Zed extensions require one.

### Phase 2: Wire in the Grammars

- Register both grammars from `jmbuhr/tree-sitter-pandoc-markdown`.
- Create `languages/pandoc-markdown/config.toml` with:
  - `name = "Pandoc Markdown"`
  - `grammar = "pandoc_markdown"`
  - `path_suffixes = ["md", "qmd", "rmd"]`
  - any basic editor settings that help Markdown authoring
- Use local dev installation in Zed to verify that:
  - the grammar loads
  - no build errors appear in `Zed.log`
  - injected inline parsing works through `pandoc_markdown_inline`

### Phase 3: Port Queries to Zed

- Start from the upstream `highlights.scm` and `injections.scm`.
- Translate highlight captures into Zed-supported capture names instead of copying them verbatim.
- Keep injections for:
  - fenced code blocks via info strings
  - HTML blocks
  - inline HTML
  - YAML metadata blocks
  - inline Pandoc markdown injection into block-level inline content
- Build a small translation pass for the initial query set:
  - `@text.title` -> `@title`
  - `@text.uri` -> `@link_uri`
  - `@text.reference` -> `@link_text`
  - `@text.emphasis` -> `@emphasis`
  - `@text.strong` -> `@emphasis.strong`
  - keep `@text.literal`, `@string.escape`, `@punctuation.delimiter`, `@punctuation.special`
  - remove or replace unsupported captures such as `@none`

### Phase 4: Add Structure for Folding and Navigation

- Add `outline.scm` centered on section-like nodes so Zed has explicit document structure.
- Prefer capturing:
  - section nodes
  - headings
  - metadata blocks when useful
  - major fenced code blocks only if they help the outline instead of cluttering it
- Add `textobjects.scm` if needed for section navigation in Vim mode and to make document sections first-class.
- Validate folding behavior early against real documents. Tree-sitter should make syntax-aware folding possible, but the exact Zed behavior should be confirmed with a prototype instead of assumed.

### Phase 5: Validate Against Real Pandoc Documents

- Test against representative files:
  - plain Pandoc markdown
  - Quarto `.qmd`
  - R Markdown `.rmd`
  - documents with YAML front matter
  - nested headings and sections
  - nested lists and block quotes
  - fenced code blocks with multiple injected languages
  - pipe tables
  - links, images, code spans, emphasis, strong emphasis, strikethrough
  - inline math and any Pandoc-specific inline constructs exposed by the grammar
- Fix the first round of bad captures, missing injections, or over-eager outline items.

### Phase 6: Package and Publish

- Add a concise `README.md` with:
  - what the extension supports
  - how file association is chosen
  - known limitations
  - local dev installation instructions
- If the upstream grammar needs fixes discovered during testing:
  - either fork it and pin your fork
  - or contribute fixes upstream and temporarily pin a forked revision

## Risks and Early Checks

### Risk 1: `.md` file association conflicts

- Zed already ships Markdown support.
- This extension is intentionally meant to supersede it for installed users.
- The practical risk is no longer whether to claim `.md`, but whether ordinary Markdown files still feel correct under the Pandoc grammar and query set.
- Early validation should therefore include both plain Markdown documents and Pandoc-specific documents.

### Risk 2: Upstream queries are not Zed-native

- The upstream grammar is useful as a base, but its highlight query set must be adapted rather than copied.
- This is likely the main quality lever for the extension.

### Risk 3: Folding behavior may need explicit structure queries

- Zed clearly uses Tree-sitter for syntax-aware editor behavior, but the extension docs explain outline queries more explicitly than folding mechanics.
- That means folding should be validated in the first prototype, not postponed.

### Risk 4: Pandoc-specific constructs may still need grammar work

- If the upstream grammar misses constructs you care about, the extension work can continue, but grammar fixes may need to happen in a fork or upstream contribution.

## Recommended Order of Work

1. Make the extension language-only and fix the manifest.
2. Register `pandoc_markdown` and `pandoc_markdown_inline`.
3. Create `config.toml` with `md`, `qmd`, and `rmd` mapped to Pandoc Markdown.
4. Port `highlights.scm` and `injections.scm` into Zed-native query captures.
5. Add `outline.scm` and validate section folding.
6. Test on real `md`, `qmd`, `rmd`, and Pandoc documents.
7. Write README and document that the extension replaces the default Markdown language, without preview support.

## Definition of Done

- The extension installs locally in Zed as a dev extension.
- After installation, `.md`, `.qmd`, and `.rmd` files open as `Pandoc Markdown`.
- Pandoc Markdown files parse with `pandoc_markdown`.
- Inline content is routed through `pandoc_markdown_inline`.
- Highlighting is clearly better than generic Markdown for both ordinary Markdown and Pandoc-heavy files.
- Section-level folding works on real documents.
- Code fences, HTML, and metadata injections behave correctly.
- The extension does not attempt to provide preview behavior.
- The repository is ready for publication with a license, README, and pinned grammar revisions.
