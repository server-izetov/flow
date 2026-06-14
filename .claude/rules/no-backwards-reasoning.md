# No Backwards Reasoning

Decisions about current code stand on current merits, not on the history of how
the code got here. Grounding a present choice in a past commit/PR/doc-comment is
forbidden — history records what was decided, not what is correct now.

Trip-wire: before reading a historical artifact (commit, PR desc, `git log` /
`git blame`, old issue), name the question. If it is "what should this code/PR
do?", STOP — history cannot answer should-questions. Applies to reading (not as
authority) AND citing (don't justify a present choice with history in prose).

Forbidden: "PR #N decided X so we must X"; "the prior PR chose Y so follow";
"kept for backward compatibility" / "compat shim" / "legacy alias" with no named
current consumer; `git blame` / `git log` as design rationale.

Plugin-compat sub-case: FLOW auto-updates — no old versions run against new
state. Forbidden: serde field aliases, key-fallback readers (`state["new"]` else
`state["old"]`), dual-key parses, compat tests. Writers produce the current
shape; readers consume it. Key-fallback to a DIFFERENT field name = forbidden;
type-tolerance accepting int/float/string for the SAME field via `tolerant_i64`
= required (`state-files.md`).

Valid history: forensic facts ("did PR #N merge?"), reading prior intent THEN
asking "still correct?", audit trails (commit messages, logs, tombstones).
Factual = fine; normative = forbidden.

Mechanical backstop (flow-plan scans issue drafts) targets four phrasings, kept
in sync with `skills/flow-plan/SKILL.md`: `"PR #<N> decided"`, `"kept for
backward compatibility"`, `"older plugin versions"`, `"as PR #<N> chose to"`.
