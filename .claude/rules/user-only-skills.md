# User-Only Skills

Six skills are reserved for direct user invocation — the model must NEVER invoke
them, nor propose them as an AskUserQuestion answer. Authorization must come from
the user typing the slash command, not inferred context.

| Skill | Why |
|---|---|
| `/flow:flow-abort` | destructive — loses in-flight work |
| `/flow:flow-reset` | destructive — wipes local `.flow-states/` |
| `/flow-release` | resource-shipping — public GitHub Release |
| `/flow-qa` | resource-shipping — files a shared QA issue |
| `/flow:flow-prime` | environment-mutating — writes project config |
| `/flow:flow-continue` | resume authorization must be explicit |

This is stricter than ask-first skills: the model does not invoke even after a
hypothetical "yes". Three mechanical layers:

- **Layer 1 `validate-skill`** — blocks a Skill call naming a user-only skill
  unless the most recent real user turn STARTS with the matching slash-command
  emission (`<command-name>/<skill>` or two-line `<command-message>`).
- **Layer 2 `validate-ask-user` carve-out** — allows AskUserQuestion mid-
  autonomous when the most recent assistant Skill call was a user-only skill (so
  abort/reset confirmation prompts fire instead of deadlocking).
- **Layer 3 `validate-claude-paths`** — blocks Edit/Write/Read/Glob/Grep across
  the `~/.claude/projects/` transcript root (tampering would defeat Layer 1); the
  auto-memory subdir is carved out.

To add one: add to `USER_ONLY_SKILLS` in `transcript_walker.rs`, this table, a
`validate_user_only_skill_<name>_is_in_set` test, and decide if its SKILL.md
needs an in-band HARD-GATE (required when reachable mid-autonomous).
