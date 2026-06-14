# Extract-Helper Branch Enumeration

When a Plan task extracts a code block into a new helper/seam/closure (triggers:
"extract X into Y", "lift/hoist/factor out X", "pull out into a seam"), the plan
must enumerate the helper's internal branches BEFORE Code phase in a Branch
Enumeration Table:

| Branch | Condition | Classification | Test |

Each branch classifies as one of three (if none fits, the extraction design is
wrong — refactor further or delete the branch):

- **Testable via seam** — inject a closure/trait/`Command`; test with a mock.
- **Testable directly** — a self-contained fixture (TempDir, state JSON).
- **Testable via subprocess** — spawn the compiled binary via the real CLI.

Constructor Invariant Audit: if the extracted block calls a panicking constructor
(`FlowPaths::new`, any `panic!`/`assert!`/`unwrap` on a param) and the input is
external-sourced, the new public surface MUST use the fallible variant (`try_new`)
— perpetuating an existing panic across the boundary still counts as a new
callsite.

If the helper recurses over a graph/tree, add a Topology Enumeration Table with a
named test per shape in the closed set: linear, tree, convergent/diamond, cycle,
depth-bounded. The diamond is the common bug: shared mutation (a `visited` set)
must run AFTER the readiness check, not before.

This is a Plan-phase artifact, NOT a Code-phase exit. A Code-phase model finding
the enumeration missing logs a deviation and proceeds. Opt-out for discussion
mentions: `<!-- extract-helper-refactor: not-an-extraction -->`.
