# No Performative Pause

During an autonomous (`continue: auto`) phase, a turn that ends with a tool call
that re-fires the loop is a continuation. Framing it as a halt — "I'm pausing",
"boundary reached", "awaiting your direction" — is dishonest, because the next
turn fires regardless. Each turn-end IS a stop; the Stop hook then queues another
turn. "Stop Refused" means the autonomous flow's end is refused, NOT that the
model cannot end a turn.

Forbidden when the same turn re-fires the loop: announcing a halt, citing an
inferred boundary, routing the next action to the user ("your call", "let me know
when you want", "ready when you are"), or naming the antipattern as if doing it
("performative pause/stop"). Honest pauses (scope genuinely complete, no
continuation tool call) are fine — say what was done and stop.

Code-phase subcase: citing a Plan-phase rule (`extract-helper-refactor`,
`scope-expansion`, `docs-with-behavior` enumeration) as permission to defer
arbitrarily-sized work is the same antipattern in rule-citation form. Plan-phase
rules are not Code-phase halt permission — log the gap per
`plan-commit-atomicity.md` "Plan Signature Deviations" and proceed.

## Forbidden Phrasings

A corpus test (`corpus_free_of_performative_pause_phrasings`) scans the corpus
(except this file) for the catalog: `I'm pausing`, `I am pausing`, `boundary
reached`, `awaiting your direction`, `let me know when you want`, `ready when you
are`, `your call.`, `your call?`, `performative pause`, `performative stop`.
Legitimate citations elsewhere use the sentinel `<!-- no-performative-pause:
legitimate-citation -->` on or above the line (per-line, no chaining).
