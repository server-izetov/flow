# Coverage Gap Diagnosis

When `bin/flow ci` reports coverage below 100/100/100, follow this fixed
sequence — substituting speculation for any step is forbidden:

1. Run `bin/test --show <file>` immediately. It marks every uncovered
   region with `^0`. Read those markers first — do not theorize. You have
   the same access to this tool as the user; running it is your job.
2. Read the source around every `^0`; identify the branch and the input
   that would exercise it. Cite file:line.
3. Read the test file for that source; a test's doc comment often names
   the gap. If a test targets the branch but misses it, the test is the bug.
4. Read sibling callers of the function; a sibling that handles the case
   is the candidate fix.
5. Only then propose a fix, referencing specific lines.

Forbidden: speculating about stale binaries / phantom coverage / profdata
races before running `--show` and naming concrete `^0` evidence;
"found it" / "the cause is" before reading the `^0` lines; asking the user
to run a diagnostic or paste output you can produce; citing a rule's
mechanism as the explanation unless you confirmed its named symptoms this
session.
