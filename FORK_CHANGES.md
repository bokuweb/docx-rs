# Fork changes (gaugo123/docx-rs)

This fork adds features MDtoWord needs, pending upstream adoption. Structured
for trivial rebasing onto new `bokuweb/docx-rs` releases.

**Upstream base:** track the tag/commit last rebased onto (update on each rebase).

## Upgrade procedure
1. `git fetch upstream && git rebase upstream/main`
2. Conflicts can only occur in the **Category-B** files below. Re-apply per notes.
3. `cargo test -p docx-rs -- --test-threads=1` ; `cargo build -p docx-rs`
4. `git push --force-with-lease origin feat/theme-color` (refreshes PR #895).
5. Rebuild MDtoWord: `cargo update -p docx-rs && cargo make ci`.

## Category A — isolated additions (NEW files, never conflict)
- `docx-core/src/documents/elements/style_ext.rs` — `Style::shading`
- `docx-core/src/documents/elements/paragraph_ext.rs` — `Paragraph::set_borders`
- `docx-core/src/types/theme_color.rs` — `ThemeColor` enum
- `docx-core/examples/theme_color.rs`
- one-line `mod` registrations in `documents/elements/mod.rs` and `types/mod.rs`

## Category B — edits to upstream-owned files (the only conflict surface)
- `documents/elements/color.rs` — 3 `Option<String>` theme fields + builders + `build_to` emission
- `xml_builder/elements.rs` — `color_with_theme` helper
- `documents/elements/{run_property,run,style}.rs` — theme delegators
- `reader/run_property.rs` — `read_color` by-attribute-name dispatch (also fixes a latent positional bug)
- `reader/run.rs` — reader round-trip test
