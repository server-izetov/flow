# Per-File Coverage Iteration

During Code phase, when the task is scoped to one `src/<name>.rs`, iterate with
the per-file gate, not full CI:

```bash
bin/test tests/<name>.rs
```

Same 100/100/100 thresholds against the mirrored src file, ~30× faster (one test
binary, not ~117). Use `bin/test --show src/<name>.rs` to inspect uncovered
regions between runs. Cross-file regressions and format/lint are caught at commit
time inside `/flow:flow-commit`'s `finalize-commit` (`ci::run_impl()`); do not run
`bin/flow ci` manually during Code phase.

Full CI is warranted only OUTSIDE Code phase, or when a change touches multiple
src files / removes a pub surface / may affect a contract test.

Phantom misses: `bin/test` runs `--no-clean`, so stale instrumented binaries can
report impossible-to-fix "missed functions" (coverage stuck despite added tests).
Diagnose with `bin/test --funcs <file>` (same demangled name at multiple crate
hashes); fix with `bin/flow ci --clean` (the one Layer 11 carve-out), then re-run.

Enforced by Layer 11 of `validate-pretool`: during active flow-code
(`current_phase=="flow-code"` AND `phases.flow-code.status=="in_progress"`) every
`bin/flow ci` variant is blocked and redirected to the per-file gate, except
`bin/flow ci --clean`. Fail-closed-as-NO-BLOCK (a state read/parse error → no
block; it's friction-prevention, not security). `finalize-commit`'s in-process
`ci::run_impl()` never hits the hook, so the commit-time gate is unaffected.
