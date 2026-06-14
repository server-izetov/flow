# Comment Quality

Comments describe the current codebase — what exists, why it exists, what
it guards. Never reference a prior implementation, deleted code, or
historical state as the explanation for current behavior.

Forward-facing test before writing a comment: "Does this make sense to
someone who never saw a prior version of this code?" No → rewrite to
describe current behavior / invariant / constraint.

Prohibited patterns:

- Parity references — "matches X", "same as X", "mirrors X" (X deleted).
- Historical provenance — "Removed in PR #NNN", "added in commit abc", "used to be X".
- Origin stories — "Port of foo.py", "based on the old implementation".
- "Before the fix" narratives — describe what the test guards, not what broke.
- "No longer" descriptions — "X no longer does Y".
- Dead section markers — "--- X removed in PR #NNN ---".

Exception: tombstone comments matching `Tombstone:.*PR #(\d+)` reference PR
numbers by design — do not rewrite these.

When fixing a flagged comment, write the new one from the code, not by
paraphrasing the old (paraphrase hides the same backward reference).
