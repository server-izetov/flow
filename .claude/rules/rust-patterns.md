# Rust Patterns

- **JSON key order**: `IndexMap` + serde `preserve_order` for any map serialized to
  JSON (else `BTreeMap` reorders state files).
- **String slicing**: `str::len()` is bytes; use `chars().count()` / `chars().take(N)`
  for code-point bounds.
- **Regex**: no lookaround in `regex`; byte-scan `as_bytes()` for ASCII operators.
- **State mutation guards**: guard `IndexMut` on string keys with `if
  !(state.is_object() || state.is_null()) { return; }`; per-level for nested; reset
  wrong-type nested keys to `json!({})`.
- **Boolean hook fields**: an `is_truthy` helper (bool, `"true"`/`"1"`, non-zero), not
  `as_bool()==Some(true)`.
- **CLI testability**: `run_impl(&Args)->Result<T,String>` + thin `run()` that
  `process::exit`s; main-arm bodies → `run_impl_main(...)->(Value,i32)` via
  `dispatch::dispatch_json/_text`; take `root`/`cwd` as params for fixtures.
- **Seam carve-out is CLOSED**: pub `_with_*` seams only for real TTY / raw-mode
  terminal / live crossterm / network socket. gh/git/bin-flow subprocess, fs reads,
  env, branch resolution are fixture-controllable, NOT seams.
- **Test subprocess stdio**: `Command::output()`, not `status()`.
- **Counters**: read via `tolerant_i64`/`_opt` (int/float/string tolerance); increment
  with `saturating_add`.
- **Empty-vs-missing**: `Some("")` ≠ `None`; `.filter(|s| !s.is_empty())`.
- **Glob**: `*` doesn't match a leading `.` unless the pattern starts with `.`.
- **Writes**: guard existence with `fs::symlink_metadata(p).is_ok()`, never
  `Path::exists()` (follows a dangling symlink and escapes the dir).
- **Dir delete loops**: skip non-file entries by `file_type`, continue-past-error,
  return deleted/skipped/failed.
- **Guard universality**: a process-level guard added to one CLI entry goes on every
  sibling in the family; read-only subcommands are exempt from `cwd_scope::enforce`.
- **Cwd-inside-destructive-path guard**: a subcommand that removes its caller's cwd
  self-gates with a canonicalize-and-compare at the top, before side effects.
- **Doc comments** on non-obvious decisions; test section markers
  `// --- primary_name ---`; log format `[Phase N] module — step (status)`.
