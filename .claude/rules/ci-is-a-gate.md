# CI Is a Gate

`bin/flow` (any subcommand) must never run in the background
(`run_in_background` is hook-blocked). Each subcommand is a CI gate or a
state mutation; it must finish and return its exit code before any
downstream action. Backgrounding defeats the gate or races the state write.

**10-minute Bash timeout.** CI-running `bin/flow` subcommands take 3–4 min;
the default 2-min Bash timeout backgrounds the process and defeats the gate.
Every SKILL.md bash block invoking one MUST carry, in the 5 non-blank lines
before the opening bash code fence (the backward walk stops at any prior
fence, so each block needs its own preamble), either `timeout: 600000` or
the phrase `10-minute Bash tool timeout`.

CI-running family (need the timeout preamble): `ci`, `start-gate`,
`finalize-commit`, `complete-fast`, `complete-finalize`, `complete-merge`.
Poll family (also long, also need it): `start-init`, `wait-for-release-ci` —
they block on a bounded real-sleep cap; on cap exhaustion re-run the single
line, never background, never fall back to a timer.
