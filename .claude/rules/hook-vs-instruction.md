# Hook vs Instruction

Use a PreToolUse hook, not a skill instruction, when: ignoring the instruction
causes a permission prompt or blocks the flow; the behavior must hold across all
phases/skills/sub-agents universally; you'd otherwise repeat the instruction in
multiple skills; or non-compliance is user-visible (blocked prompt, wrong file
edited, permission denied).

Use a skill instruction when the behavior is specific to one phase/step,
non-compliance is annoying but not flow-blocking, or it's contextual (depends on
plan content, user prefs, phase state).

The principle: skill instructions are advisory — the model can ignore them. Hooks
are enforcement — they run as code before the tool executes. If "the model might
not follow this" has a consequence that blocks the user, it must be a hook.

The mechanically-enforced gates (compound-command/redirect blocking, escape-hatch
layer, the Layer 10 commit gate and its carve-outs, the Layer 11 ci-during-code
redirect, the halt gates on Skill/Bash, the autonomous Stop refusal, the
AskUserQuestion autonomous block + carve-outs, the `.claude/` and transcript-root
path gates, the shared-config gate) live in `src/hooks/*.rs` and are documented at
their source and in the topical rules (`concurrency-model.md`,
`autonomous-phase-discipline.md`, `user-only-skills.md`,
`per-file-coverage-iteration.md`). When a hook block surprises you, use the
matcher→hook map in `hook-error-diagnosis.md`.
