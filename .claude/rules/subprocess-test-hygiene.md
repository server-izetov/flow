# Subprocess Test Hygiene

Every test spawning the project binary (`Command::new(env!("CARGO_BIN_EXE_..."))`
or any process reading the ambient env) must deliberately neutralize the surfaces
its subject code reads — the default "inherit everything" is wrong for tests:

- **Network creds** for any service the code might reach: `GH_TOKEN` /
  `GITHUB_TOKEN` → `"invalid"` (gh fails auth fast, no hang/mutation); Slack vars
  → empty/remove; cloud creds → remove.
- **Ambient config**: `HOME` → the test's tempdir root (no user dotfiles).
- **Recursion guard**: `env_remove("FLOW_CI_RUNNING")` for CI-tier subcommands.
- **Coverage**: keep `LLVM_PROFILE_FILE` valid by setting `current_dir` under the
  target dir, or rely on the `.gitignore` + sweep safety net.

**Machine-global per-session markers under HOME** (`phase-enter`,
`set-utility-in-progress` write `<HOME>/.claude/flow/...-<session_id>`): set BOTH
`HOME=<fixture>` AND `env_remove("CLAUDE_CODE_SESSION_ID")` — else a suite run
inside an active flow overwrites that flow's live marker and corrupts
`--continue-step` resume.

**Working-directory isolation**: a test spawning a binary that reads the state
file (hook validators especially) MUST `.current_dir(fixture_root)` pointing at a
dir that does NOT resolve to an active flow — inheriting the runner's cwd couples
the outcome to whatever flow is active. Hook subprocess tests route through the
shared `crate::common::spawn_hook` helper.

A plan adding a subprocess test names the services its subject reaches and the
neutralizers; an omitted one is a Plan-phase gap.
