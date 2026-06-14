# Ephemeral File Cleanup

A FLOW phase that writes an ephemeral worktree-internal artifact (the Review
adversarial probe is canonical) has exactly one creating phase and one disposing
phase. Disposal is an explicit `fs::remove_file` from inside Rust — not a side
effect of `git worktree remove`.

Rules for any such cleanup step (`src/cleanup.rs`):

- **Order before worktree removal.** A step removing a worktree-internal file
  must run BEFORE `git worktree remove` (afterward the path is gone). Files
  under `<project_root>/.flow-states/` are outside the worktree and may run
  after.
- **Delete via `fs::remove_file`** — not `rm` (allow-list-blocked mid-flow), not
  `git rm` (fails on excluded/untracked files), not a Write-tool stub (leaves a
  file). Discover the path via a project-owned hook (`bin/test
  --adversarial-path`), not a hardcode.
- **Tolerate NotFound** (cleanup may run twice): `Ok(())` → "deleted",
  `NotFound` → "missing", other `Err` → "failed: <e>", path didn't resolve →
  "skipped". Emit the outcome as JSON so disposal is audit-trailed.
- A new ephemeral artifact adds a per-artifact step before worktree removal with
  these outcomes and per-outcome tests in `tests/cleanup.rs`.
