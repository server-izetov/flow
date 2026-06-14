# Verify the Runtime Path

Before writing a fix, trace the actual execution path:

1. Find the real call site — grep all callers; a function may be called from
   elsewhere, or not at all if another path runs first.
2. Verify runtime behavior — run the real chain (Claude Code → bash → bin/flow
   → flow-rs) and print actual values. Unit-test mocks miss environment issues
   (missing tty, wrong parent, piped stdin).
3. Check one layer deeper — an unexpected subprocess value (`??`, empty, wrong
   PID) is a symptom; investigate before filtering it out.

Plan phase, when a plan adds a NEW production path — a new branch in an
existing function, OR a new function/scanner/helper wired into multiple entry
points — enumerate every caller/invocation site that takes the new path and,
per row: the conditions that hit it, and the named test that exercises it with
those inputs. Never collapse multiple callsites into "the N callsites".

Forbidden: committing a fix without running the real path; stacking a second
fix on an unverified first; trusting unit tests when the bug is environmental;
assuming which file owns state without grepping all writers.
