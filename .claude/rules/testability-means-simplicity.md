# Testability Means Simplicity

Tests exist to (1) prove it works, (2) detect over-engineering — a branch hard
to cover means the code is more complex than the problem requires; the
un-testable shape is the bug, not the test gap — and (3) guard regression.

Reason (2) is the diagnostic. When a coverage gap resists straightforward tests
— needs mock traits, subprocess-only paths, monomorphization hunts, elaborate
seam injection — stop writing tests and simplify the code. First run the
`.claude/rules/reachable-is-testable.md` triage: simplify only when it surfaces
an over-engineered branch with no legitimate public consumer; if the production
path needs a fixture the test lacks, fix the test, not the code.

Over-engineering signals: a new trait+mock to drive one error branch; an Err
region covered in one codegen unit but invisible in another; a fixture needing
a fake `$PATH` / non-executable binary / signal-killed child; a fixture longer
than the function; a function existing only as a seam; reaching for
`#[inline(always)]` / `#[cfg(test)]` to close a monomorphization gap.

Fix: describe the function in one sentence (an "and"/"with" means it does too
much); pick the simplest stdlib primitive (`Command::output()`,
`fs::read_to_string()`, a `match` ladder over a trait seam); delete infra that
existed only for testability (`_with_runner` / `_with_deps` / mocks whose only
caller is tests); rewrite tests against the simpler function.
