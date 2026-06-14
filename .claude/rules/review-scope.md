# Review Scope — All Real Findings Fixed In PR

Every real Review finding is fixed in Step 4. Triage has two outcomes: Real →
fix; False positive → dismiss with specific rationale citing code. There is NO
filing path — filing a real finding as out-of-scope is forbidden. Mechanically:
`bin/flow add-finding --phase flow-review` accepts outcome only in
{fixed, dismissed}; `bin/flow issue` is blocked when `current_phase ==
"flow-review"` (escape: `--override-review-ban`).

Before classifying, run the `supersession.md` test — code the PR made redundant
is deleted, not filed.

Value-vs-bureaucracy: a Real classification must survive — would the fix add
signal a reader can't already derive from code/rules/diff? If the fix only
duplicates discoverable info (a redundant doc / CLAUDE.md / cross-ref), it is a
False positive even if technically correct. Apply most rigorously to
documentation (tenant 6); tenants 1–5 (behavior/security/correctness/coverage)
almost always pass. When in doubt, surface to the user.

A new rule added in the PR that retroactively flags pre-existing violations →
those are still Real, fixed here (see `scope-expansion.md`). A rule or skill that
lands on the base branch mid-flow → decide proactive-sweep vs defer-to-Review and
LOG the decision via `bin/flow log`.
