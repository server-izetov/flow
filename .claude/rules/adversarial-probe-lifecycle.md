# Adversarial Probe Lifecycle

Review's adversarial agent writes a throwaway test probe at the path from
`bin/test --adversarial-path`. The probe is NEVER tracked by git: it is in
`src/prime_check.rs::EXCLUDE_ENTRIES` (so `git add -A` never stages it) and
`tests/structural.rs::adversarial_probe_must_not_be_tracked` asserts
`git ls-files` is empty. A tracked stub would survive worktree cleanup and
persist on the integration branch — forbidden.

The probe is disposed at the Complete phase by
`src/cleanup.rs::delete_adversarial_probe` via `fs::remove_file`, BEFORE
`git worktree remove` (afterward the path no longer resolves).

When Review Step 4 fixes (or Step 3 dismisses) a finding the probe surfaced, the
probe's assertions are stale and fail CI. Reconcile in the same pass — default:
DELETE the probe (findings are already recorded via `add-finding`, and named
regression guards live in `tests/<path>/<name>.rs`). Update its assertions only
when the new behavior itself needs a guard that belongs in the probe (rare). The
path must not be left tracked.
