# Forward-Facing Authoring

Every persistent artifact — `.claude/rules/*.md`, `CLAUDE.md`, Rust doc
comments, SKILL.md, plan prose, finding reasons re-read as guidance —
describes what IS, not what WAS. The motivating incident is fixed; readers
do not reconstruct it.

Test before writing: "Does this make sense to a reader who never saw the
incident that produced it?" No → rewrite to the principle/invariant; remove
the history.

Prohibited: first-person narrative ("we hit this", "during PR #NNN"); "after
the fix" framing; "was/was-not" / "previously the code did X"; citing a
file/PR the session just touched as the anchor; principles drawn from one
incident with no generalization; appended motivating-incident sections.

Correct: name the code shape abstractly + a generic snippet; state the
invariant/contract in present tense ("When X, do Y"); cite code only as a
permanent canonical example.

Exempt (historical by nature): commit messages, issue bodies, session logs,
state-file `findings[].reason` triage fields, tombstone comments.

When turning a backward-facing finding into a rule, strip the incident. If
the rule only makes sense with the incident, it is too specific — record it
in the commit message and discard.
