# File-Tool Preflights

Claude Code's Write errors if the target pre-exists unread; Edit errors if the
target wasn't Read this session. Avoid both:

- **Monitored persistent paths route through `bin/flow write-rule`** (which does
  unconditional `fs::write`): `.flow-states/<branch>/plan.md`, `.flow-issue-body`,
  `orchestrate-queue.json`. Write content to
  `.flow-states/<branch>/<purpose>-content.<ext>` with the Write tool, then
  `bin/flow write-rule --path <target> --content-file <that>`. (Session-scoped
  `-<id>` temp files are exempt — unique names don't collide. The commit-msg file
  is exempt — finalize-commit deletes it each exit.)
- **Edit on a named plan file** must be preceded by an explicit Read-tool
  instruction on the same file (satisfies the Edit preflight on `--continue-step`
  resume).

`.flow-states/` lives ONLY at `<project_root>/.flow-states/`.
`validate-worktree-paths` enforces it: a misplaced worktree-internal
`.flow-states/` Write/Edit is silently rewritten (PreToolUse `updatedInput`) to
the canonical path; a misplaced Read/Glob/Grep is BLOCKED with the canonical path
named. `bin/flow write-rule` also self-canonicalizes managed artifacts (`plan.md`,
`.flow-issue-body`, `orchestrate-queue.json`): a misrouted `--path` is rejected
(exit 1, `step:path_canonicalization`) before reading the content file;
non-managed basenames pass through (the `.claude/rules/` write path depends on it).

Enforced by two `tests/skill_contracts.rs` tests (Write→write-rule adjacency;
Edit→preceding Read) and the `tests/write_rule.rs` canonicalization matrix.
