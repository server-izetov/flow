# Test Placement

Every test lives at `tests/<path>/<name>.rs` mirroring `src/<path>/<name>.rs` (pure
`s/^src/tests/`), and drives the subject through its PUBLIC interface. No inline
`#[cfg(test)]` blocks or test-only items in `src/*.rs`.

A branch reachable only via a private helper means one of: the branch has no public
path (delete it), or the public surface is missing a seam (add the seam). Never make
an item `pub` solely to test it.

- Cargo auto-discovers test binaries — never add `[[test]]` to `Cargo.toml`. Subdir
  tests: one binary per `tests/<subdir>/main.rs` declaring siblings as `mod <name>;`;
  shared helpers via `#[path="../common/mod.rs"] mod common;` in main.rs, referenced
  as `crate::common::*`.
- Meta-tests with no src mirror live at `tests/` root as listed exceptions
  (`test_placement`, `tombstones`, `skill_contracts`, `structural`, `permissions`,
  `docs_sync`, `rule_authoring`, the `bin_*` / `binary_artifact` / `hello_smoke`
  contracts). Adding one amends that list.

Bright-line for a `pub` addition: name the non-test production consumer outside the
module, in the commit message AND the item's doc comment. If the only callers are a
same-module dispatcher + tests, it's pub-for-testing and forbidden. Forbidden as pub
unless a named cross-module consumer exists: `_inner`, `_impl`, `_with_runner`,
`_with_resolver`, `_with_deps`, `_with_tty`, `run_impl_main_with_*`. The ONE carve-out
is externally-coupled seams (real TTY, raw-mode terminal, live crossterm loop,
network socket) — CI runners, git/gh subprocess output, state/sentinel reads are
fixture-controllable, NOT externally coupled.

Enforced by `tests/test_placement.rs::src_contains_no_inline_cfg_test_blocks` (the
one escape for a needed literal is `concat!("#[cfg", "(test)]")`).
