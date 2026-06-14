# Branch Path Safety

A branch name from outside the process (`--branch` flag, `current_branch()`/
`resolve_branch()` git output, state-file read, env var) MUST be validated
before interpolation into any `.flow-states/` or `.worktrees/` path. The
validator `FlowPaths::is_valid_branch` rejects: empty, `.`, `..`, any `/`,
any `\0` — each escapes the per-branch dir (cleanup runs
`fs::remove_dir_all(branch_dir())`, so the blast radius is unbounded).

Reach the filesystem through one of two guards:

1. `FlowPaths::try_new(root, branch)` → `None` on invalid. External-source
   callers (CLI, git output, hooks) pattern-match `None` as "no active flow"
   (early return / structured error / skip). A caller holding a branch
   validated upstream may chain `.expect("<boundary naming the sanitizer>")`
   — documentation, not a panic vector.
2. `FlowPaths::is_valid_branch(&branch)` pre-validation before any path
   construction; reject with a structured error on false.

Forbidden: `format!(".flow-states/{}", branch)` or `format!(".worktrees/{}",
branch)` without a guard. Enumerate every hook AND CLI callsite taking the
same branch input; guard them all. Canonical guards: `src/flow_paths.rs`.
