# Config Source Mapping

When a plan task modifies a config file (`.claude/settings.json`,
`.flow.json`, `flow-phases.json`, `hooks/hooks.json`, etc.) or claims a
derived value (hash/checksum/generated artifact) changes as a result, the plan
must cite — in Tasks or Risks — the specific Rust code / hook / consumer that
reads the changed value at runtime (file + function/line), and for any claimed
downstream effect, name the computation and verify the changed value is in its
input set. Verify with a grep + Read before asserting the effect.

Canonical config→reader mappings (load-bearing facts):

- `compute_config_hash` reads ONLY Rust constants in `src/prime_check.rs`:
  `UNIVERSAL_ALLOW`, `FLOW_DENY`, `EXCLUDE_ENTRIES` — NOT `.claude/settings.json`.
  A settings.json change affects the hash only if mirrored in the constant.
  Pinned: `tests/prime_check.rs::compute_config_hash_uses_python_default_formatter`.
- `.claude/settings.json` — read by Claude Code at runtime, never by Rust;
  changes take effect immediately for the session.
- `.flow.json` — read at flow-start, copied into the state file; the running
  flow reads prefs from `.flow-states/<branch>/state.json`, not `.flow.json`.
- `flow-phases.json` — read by `check-phase`/`phase-enter`/`phase-finalize`/
  `phase-transition`; next-invocation effect.
- `hooks/hooks.json` — read by Claude Code at session start; needs a new session.
- `assets/bin-stubs/<tool>.sh` — read by `prime_setup.rs`; the
  `# FLOW-STUB-UNCONFIGURED` marker is read by `ci.rs::any_tool_is_stub`;
  affects only NEW primes.
