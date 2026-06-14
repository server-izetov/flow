# Plan Commit Atomicity

A plan's "atomic commit group" may be split across commits only when ALL three
hold: each intermediate commit passes `bin/flow ci` independently; no test
assertion spans the boundary (none asserts content one commit removes and another
re-adds); and the split reflects a meaningful logical boundary, not a
context-budget convenience. Else commit the group as one.

The inverse — COMBINING non-atomic tasks into one commit for efficiency (identical
shape, shared CI snapshot) — is permitted but must be LOGGED at the moment via
`bin/flow log "[Phase 2] Batch decision: combining Tasks N-M (...) for <reason>;
each independently shippable, no assertion spans the boundary."` Not permitted
when a test in one task asserts another's behavior, or the plan listed them
separate with distinct commit subjects.

**Plan signature deviations must be logged.** Any Code-phase deviation from a
plan-level interface prototype — function signature, type shape, file name, task
count — must be recorded via `bin/flow log "[Phase 2] Plan signature deviation:
<old> -> <new> (<why>)"` before the commit delivering it, AND in the commit-message
body. Required for added/removed params, changed return types,
renamed-with-different-intent, struct field changes, task count/order changes,
file renames. NOT required for typos, whitespace, in-scope test renames, or
implementation-detail changes under an unchanged signature (equivalent syscall,
restructured match arms, swapped stdlib primitive — same observable behavior).

Mechanically backstopped: `plan_deviation::scan` inside `finalize_commit::run_impl`
cross-references plan-named `(test, fixture_key, plan_value)` triples against the
diff's added test bodies; an intentional deviation clears via a `bin/flow log`
line naming the test and the plan value.
