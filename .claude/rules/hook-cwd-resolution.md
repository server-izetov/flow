# Hook cwd Resolution

PreToolUse enforcement hooks resolve the working directory they reason about
from the hook payload's `cwd` field via `crate::hooks::resolve_hook_cwd` —
never from the subprocess's own `std::env::current_dir()`. Claude Code passes
the session/sub-agent cwd (the worktree, during a flow) in the payload; the
subprocess's own cwd can be the main repo root, which silently self-disables
every worktree-derived gate (validate-worktree-paths, validate-claude-paths,
validate-skill, validate-pretool's branch / main_root / flow_active /
agent-prompt / Layer-10-11 / halt consumers).

```rust
let cwd = hook_input.as_ref().and_then(crate::hooks::resolve_hook_cwd);
```

`resolve_hook_cwd` returns payload `cwd` when present and non-empty, else falls
back to `env::current_dir()` (empty string treated as absent). Thread the one
resolved value to every cwd consumer so they cannot diverge.

Carve-out: the Agent-prompt scan (`validate_agent_prompt`) allows candidate
paths under THIS flow's own `<project_root>/.flow-states/<branch>/` subtree
(scoped to the branch, not the whole `.flow-states/` root every concurrent flow
shares), so legitimate Review sub-agent launches carrying the substantive-diff
path there are not hard-blocked.
