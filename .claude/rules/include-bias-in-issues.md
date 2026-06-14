# Include Bias in Issues

**Default to inclusion when filing or scoping an issue.** The question is not
"should this be in scope?" but "is there a CONCRETE reason this must NOT be?"
Absent a concrete blocker, adjacent concerns belong in scope. The lifecycle cost
favors it: including is O(1) (one task in the current exploration budget);
splitting is O(N) (a whole new Plan→Code→Review→Complete lifecycle re-exploring
the same files).

Not valid exclusion reasons: "the prior PR didn't touch this"; "the user owns
this"; "separate code surface"; reflexive "would expand scope" (instead apply
the three-condition gate in `scope-expansion.md`); a defensive "Out of Scope"
list written before any concrete blocker surfaced.

Valid exclusions (rare): the user explicitly rejected the scope; including needs
a different design conversation (new architecture/security questions); including
would block the issue's primary completion (then file the follow-up now and link
via `bin/flow link-blocked-by`). Write the rationale as one prose sentence naming
the concrete blocker — not a bulleted exclusions section.

Mechanical backstop (flow-plan scans drafts) targets four phrasings, kept in
sync with `skills/flow-plan/SKILL.md`: "Out of scope", "Non-goals", "would
expand scope", "separate code surface".
