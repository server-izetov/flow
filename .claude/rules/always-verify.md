# Always Verify

A change is complete when its check passes, not when the edit lands.
A change reported as done must name the verification that confirmed it
(typically a command's exit code). "Should work" is not done; a red
check is a blocker, not a status update.

Verification command by change class:

- Rust source: `bin/flow ci` (or the narrowest single-phase variant
  `--format`/`--lint`/`--build`/`--test`).
- A single test: `bin/flow ci --test -- <name>`.
- Skill / rule / CLAUDE.md content: the contract test covering it, via
  `bin/flow ci --test -- <test_name>`.
- Toolchain/permission config (`.claude/settings.json`,
  `.config/nextest.toml`, `Cargo.toml`, `hooks/hooks.json`): `bin/flow ci`.

Pick the narrowest check that fully exercises the change; widen when blast
radius is uncertain. Format/lint are deterministic — apply the emitted fix
and re-run, no judgment call.
