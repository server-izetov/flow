# Docs With Behavior

When a change modifies behavior that documentation describes, update the docs in
the SAME commit — never a follow-up issue (the next session would re-read the code
to write the same update).

## What Counts

- Changed skill steps/flags → `docs/skills/<name>.md`.
- Changed phase behavior → `docs/phases/phase-<N>-<name>.md`.
- New CLI subcommand / changed state mutation → `CLAUDE.md` + `flow-state-schema.md`.
- Changed state field range/total/name → `flow-state-schema.md`.
- Changed what a skill passes a sub-agent → that agent's `## Input` section.
- New field/line in a formatter's output → the SKILL.md describing its panel.
- New permanent on-main artifact → `CLAUDE.md` "Key Files" (name + 1-line purpose only).
- Changed type signature / module architecture → the module doc comment and every
  affected item's doc comment in the same source file.

Feature-configurable prose generalization: when code adds a configurable parameter
where a value was hardcoded, grep the whole prose corpus for the old value and
generalize universal prose (applies to every project) in the same PR; leave
self-referential prose (describes THIS repo) alone.

Deletion-side scope enumeration: grep EVERY old identifier the change removes
(symbol name, prose phrase, layer/step number). For a deleted state-machine
element, build the two-column exercise-vs-reference test inventory and pre-classify
every test (mechanical-retarget / delete / rewrite) before Code phase.

Multi-task plans: mark the behavior change + its doc update as an atomic group (or
one task) so they land together. Review fixes all PR-introduced doc drift in-PR.
