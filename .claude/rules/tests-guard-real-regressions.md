# Tests Guard Real Regression Paths

Every test guards a specific regression with a named consumer. Before adding one,
state: (1) the exact change that would break the asserted property, (2) the code
path that produces that change, (3) the rule/skill/hook/test that relies on the
property. If you can't name all three, it's speculation — delete it.

Forbidden: "just in case" / "for future drift" scans without a named regression;
duplicate guards for a property an existing tombstone/contract test already
covers; corpus-wide substring scans whose only occurrences are in files that must
legitimately use the term.

Coverage-required tests are NOT speculation: the 100% gate is the named consumer.
Scope one test per branch; verify it trips when the covered line is deleted
(comment out the line, confirm red, restore).

Multi-file contract tests: default to per-file siblings (each names its own
regression; failure output names the file). A single coordinated test is allowed
only when the invariant is genuinely cross-file, with a doc-comment stating why
splitting loses the property, the per-file regression each branch guards, and the
canonical file to read first on failure.

Before adding a corpus contract test, run the viability check: apply the
candidate vocabulary to the corpus; ≤4 flags → audit/fix each; ≥5 → defer the
test (intrinsic false positives) and document the deferral with the count.

Frozen-golden tests (pinned hash/snapshot) guard byte-stability for a persisted
consumer; verify the golden value independently before pinning and document the
update protocol.
