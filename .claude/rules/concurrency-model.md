# Concurrency Model

N engineers × N flows × N machines run simultaneously. Before writing code, ask
"what if two flows hit this at once?":

- **File paths** must be branch- or worktree-scoped — never a fixed `/tmp/<name>` or
  repo-root singleton. Use `.flow-states/<branch>/*` or worktree-local paths.
- **State mutations** stay isolated to the current flow's state file.
- **GitHub ops** must be idempotent (labels, PR/issue updates may race) —
  last-write-wins or check-before-write.
- **Locks** only serialize shared resources. The base branch (integration trunk) is
  the ONLY shared local resource; any base-branch op (pull/commit/push) is serialized
  via the start lock. Acquire and release a lock under the SAME name (resolve the
  canonical branch before acquiring).
- Start-gate runs CI on the base branch under the start lock as a coordination
  surface (repair once via ci-fixer, others inherit via the sentinel).

A completed flow's state file may survive cleanup failure; scanners for active flows
must skip files where `phases.flow-complete.status == "complete"`.

Editing source on the base branch: default NEVER (go through a feature branch).
Exceptions: the user explicitly directs an on-base fix this session; or the three
bootstrap skills that commit on the trunk by design (`/flow:flow-start` ci-fixer,
`/flow:flow-prime` setup, `/flow-release` version bump).

Mechanical enforcement (`validate-pretool` Layer 10): direct `git commit` /
`bin/flow finalize-commit` is blocked when the destination resolves to the
integration branch OR a feature branch with an active state file. Carve-outs allow
only sanctioned skill paths (active-flow `/flow:flow-commit`, the bootstrap skills, a
user-typed `/flow:flow-commit` on trunk from a non-active-flow cwd). Raw `git commit`
is never carved out; user direction does NOT lift the gate — route through
`/flow:flow-commit`.
