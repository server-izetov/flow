# Autonomous Phase Discipline

During a `continue: auto` phase, introduce no user-facing pause the user didn't
request:

- Never emit `AskUserQuestion` for self-invented checkpoints ("proceed?",
  "continue?"); never self-declare a context/budget check; never decide the flow is
  "too big" and ask. Autonomy means the user already answered.
- Never mark a counter (`code_task`) complete then halt without committing the work.
- Never end the turn voluntarily without a tool call. The Stop hook
  (`check_autonomous_stop`) refuses a turn-end during an in-progress auto phase; a
  text-only stop is queued back as another turn. Never frame a re-firing
  continuation as a halt (`no-performative-pause.md`).

If context is genuinely exhausted, commit in-flight work at a natural boundary then
stop — never pause where nothing was committed.

Under auto, NO permission prompt may reach the user either: a legitimate op hitting
a sanctioned-tool gap → extend the sanctioned surface; a model reaching for an
unsanctioned op (transcript-JSONL read, placeholder anchor) → remove it at source.

Two-exit halt model: when a real conversational-prose user turn appears mid-flow,
the Stop hook sets `_halt_pending` and allows ONE stop to answer; every later stop
is refused until the user types `/flow:flow-continue` (resume) or `/flow:flow-abort`
(give up) — both user-only. `_halt_pending` and the stalling counters
(`_last_observed_code_task`, `_consecutive_unchanged_count`) are in
`MODEL_DENIED_FIELDS` (the `set-timestamp` CLI cannot write them). Halt also gates
Skill calls (`validate-skill` Layer 2) and flow-advancing Bash. Forbidden stalling
frames: "a rule authorizes me to halt" (none does), "this task looks bigger so it's
harder", "writing about the work substitutes for doing it". On `/flow:flow-continue`
resume from where the halt landed — don't re-survey or re-confirm.
