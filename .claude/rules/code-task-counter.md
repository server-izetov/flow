# Code Task Counter Convention

`code_task` in `.flow-states/<branch>/state.json` tracks the Phase 2 plan-task
counter, advanced via `bin/flow set-timestamp --set code_task=<n>` after each
task, before commit. It increments **once per plan task**, regardless of commit
grouping — a test+implementation TDD pair is two tasks, so the counter advances
twice even when both land in one commit. (Readers: the resume check picks the
next task as `code_task + 1`; the audit compares to `code_tasks_total`. An
under-count breaks resume and produces a false process-gap finding.)

For an atomic commit group, batch the advances in one call — each `--set` is
validated +1 against the prior `--set`'s in-memory state:

```text
bin/flow set-timestamp --set code_task=4 --set code_task=5 --set code_task=6
```

Advance monotonically only — never jump. When a later task's tests must land
early (coverage forces Task N's tests into Task M's commit, N>M), advance only
through the contiguously executed prefix (to M), log the early-landed task via
`bin/flow log "[Phase 2] Plan deviation: Task N tests landed early..."`, and
catch the counter up to N when execution reaches N in planned order.
`set-timestamp` enforces +1 per `--set`; a jump (5 when current is 0) is rejected.
