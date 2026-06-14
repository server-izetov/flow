# Worktree Commands

- **Use `git -C <path>` not `cd <path> && git`.** Claude Code's "bare
  repository attacks" heuristic fires on any `cd <path> && git` compound
  command regardless of allow list. `git -C` matches `Bash(git -C *)`.
- **Worktree file paths.** When pwd contains `.worktrees/`, ALL file-tool
  paths (Edit/Read/Write/Grep/Glob) for repo-tracked files must use the
  worktree absolute path from `pwd`, not the main-repo path — the worktree has
  its own copy. Verify each Edit/Write path starts with `pwd`. Shared paths
  OUTSIDE the worktree (`.flow-states/`, `~/.claude/`, plugin cache) are
  accessed directly.
- **Never invoke `cargo` directly.** No `cargo test`/`build`/any subcommand
  via Bash — use `bin/flow ci` (or `--format`/`--lint`/`--build`/`--test`,
  `--test -- <filter>`). Direct cargo bypasses the whitelist → RTK prompts the
  user, especially dangerous inside sub-agents.

`bin/flow write-rule` is gated at the subprocess layer
(`write_rule.rs::worktree_path_guard`): during an active flow it rejects a
protected-basename `--path` not under `<main_root>/.worktrees/<branch>/`,
naming the canonical destination.
