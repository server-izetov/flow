# Hook State Read Timing

When a PreToolUse hook (or any mid-session code) reads FLOW state fields
written by phase-transition commands (`current_phase`, `phases.<N>.status`,
`_auto_continue`, `_continue_pending`), trace the read window against the write
path before designing the gate — those fields mutate at specific lifecycle
moments and a hook that ignores WHEN fires in unintended states.

Failure shape: `phase_complete()` advances `current_phase` to the NEXT phase
BEFORE the completing skill's HARD-GATE fires; a hook gating on bare
`current_phase` blocks the approval → deadlock. Fix: add a phase-status
predicate (`phases.<current_phase>.status == "in_progress"`, set by
phase_enter, cleared by phase_finalize) so the block fires only while the phase
is actively executing — never gate on an advancing field alone.

Plan-phase Risks must enumerate, for the hook: (1) every field it reads;
(2) each field's writer(s) — grep `src/` for `state["<field>"] = ` (writers:
`phase_transition.rs`, `phase_enter.rs`, `phase_finalize.rs`,
`commands/init_state.rs`, `set_blocked`); (3) the temporal ordering vs.
observable skill actions; (4) the read window. If the hook can fire between
any two mutations, gate on a stable marker instead of the advancing field.
