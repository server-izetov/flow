# External-Input Path Construction

When a state-derived string (state-file value, env var, parsed JSON, CLI flag,
hand-edited config) flows into path construction (`format!`, `Path::join`,
`PathBuf::from`, `fs::*::open`), it must pass a POSITIVE validator BEFORE the path
is built; and every read of a caller-controlled/state-derived path must enforce a
documented byte cap.

- **Validate before constructing.** A positive validator (`is_safe_<purpose>`)
  rejecting empty, traversal segments (`.`, `..`), separators (`/`, `\`), NUL, and
  anything outside the expected closed set.
- **Prefix-contain when the value is itself a path** — require absolute AND rooted
  under a known dir (`<home>/.claude/projects/`, `<root>/.flow-states/`); reject
  anything outside.
- **Byte cap on every read** — `BufReader::new(file.take(CAP))`, CAP a module
  `const` with a doc comment naming the worst case. Applies to direct reads AND
  every per-entry read in a directory walk.
- **Env-var paths must be absolute** — reject empty/non-absolute `$HOME` etc.
  before joining (a relative value resolves against the worktree).

Reference: `src/session_metrics.rs` (`is_safe_session_id`,
`is_safe_transcript_path`, `TRANSCRIPT_BYTE_CAP = 50 MB`).

A plan proposing a new external-input read OR a filesystem walk must name all
four: source, sink, validator (per-entry filter for walks), byte cap.

No `.expect()` on `fs::read_dir`/`read_to_string`/`File::open`/`symlink_metadata`
in hooks (`src/hooks/*.rs`) or CLI `run_impl` — a panic surfaces at the user's
terminal. Use `match`/`?`/`.ok()` + fallback; walks swallow errors to `continue`.
