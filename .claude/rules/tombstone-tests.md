# Tombstone Tests

When intentionally removing a named feature / config axis / numbered step /
external dependency, add a test asserting the removed identifier does NOT appear
in the source. This converts deletion intent from invisible absence into a
CI-failing presence check (catches merge-conflict resurrection).

Comment format `Tombstone: <text> PR #<N>` (only `PR #<number>` is recognized by
`tombstone-audit`; cite the merge PR). Naming: `test_<scope>_no_<removed_thing>`.

Two kinds:

- **Stable source literal** (a quoted CLI arg, function name, config key) →
  byte-substring `!content.contains("X")` is OK ONLY if X cannot be reassembled
  via `concat!`, `format!`, a named constant, or split `.arg()` calls and still
  take effect. Document why it's stable (the four-question checklist).
- **Structural construct** (a subprocess call, a deprecated API — many source
  shapes) → scan the protected function's body (bounded `split_once`) for the
  construct, not a literal. Assume this kind when in doubt.

Always pair a deleted source file with a file-existence tombstone (`git ls-files`
empty) AND the byte-substring — else `#[path=...]` re-import resurrects it.

A removal plan must specify per tombstone: protection target, assertion kind,
stability argument, bypass list, file-resurrection pair. Consolidated tombstones
live in `tests/tombstones.rs`; topical ones stay in their domain test file.
Removal is via `bin/flow tombstone-audit` (stale once the removal PR merged
before the oldest open PR was created).
