# Skill Authoring

- **Simplest approach first.** Don't add machinery (resume checks, self-invocation,
  counters) unless you can state in one sentence why the simple approach fails.
- **Flat `### Step N` numbering** — no sub-steps (1a/2b); group with a prose preamble.
  Use bold paragraph headers (`**Step.**`) not numbered lists when steps contain
  fenced code blocks. Blank line after every fenced block (incl. before closing tags).
- **Every user decision point is wrapped in `<HARD-GATE>`** with explicit enforcement
  language — prose "ask the user" is insufficient. A flag that bypasses a gate needs a
  matching carve-out in Hard Rules.
- **`bin/flow` prefix**: marketplace skills (`skills/<name>/`) prefix `bin/flow`
  with the plugin root env var `CLAUDE_PLUGIN_ROOT`; project-local skills
  (`.claude/skills/<name>/`) use bare `bin/flow` (contract-tested). For
  repo-modifying subcommands in a worktree, use the worktree's own `bin/flow`.
- **Repo-tracked file paths** say "current working directory", never "project root"
  (in a worktree the project root is the main repo; the hook blocks it). Project root
  is correct only for `.flow-states/` and other shared artifacts.
- **Platform constraints**: `.claude/` paths are protected regardless of settings;
  Edit/Write redirect to `bin/flow write-rule`, and `mkdir` under `.claude/` is
  forbidden — `write-rule` creates parent dirs itself.
- **Sub-agents**: never `general-purpose` (ignores tool restrictions), never
  `bypassPermissions`; use custom plugin agents + the global hook.
- **Commit skill** owns `.flow-commit-msg` end to end; parents never write it; always
  `git add -A` before `git diff --cached`.
- **State-dependent gate ordering**: a step's gate command needs its state field
  written by a PRIOR step, in the SAME step (no `### Step` between mutation and gate);
  add a contract test asserting both textual order and adjacency.
- **Mid-phase self-invocation** (not HARD-GATEs) to continue after a built-in Skill
  returns: re-invoke the skill with `--continue-step`, dispatching off a state counter.
- **Plan task ordering**: test tasks before implementation; a removal needs a tombstone
  task first. Audit CLAUDE.md and `AUTO_SKILLS` on any rename/reorder. Every
  `CONFIGURABLE_SKILLS` skill needs an entry in all 4 prime presets (CI-enforced).
- **Verify issue/plan references** (skill dirs, test functions, script behavior,
  command names) exist via grep/glob before building on them.
