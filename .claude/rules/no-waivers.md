# No Waivers — 100% Coverage, No Escape Hatch

All Rust code must be covered. There is NO waiver mechanism: a `test_coverage.md`
/ `security_waivers.md` per-line exception file is forbidden — the file and the
discipline that authorizes one.

When a path seems unreachable from in-process tests, the only responses are:

1. Add a subprocess test spawning the binary (`tests/main_dispatch.rs` pattern).
2. Refactor for testability (the `run_impl_main(...) -> (Value, i32)` seam).
3. Delete the branch if no production caller reaches it.

If none works, the code is wrong, not the test surface.

Forbidden in plan prose (these ARE proposing waivers): "add a test_coverage.md
entry…", "if any line remains uncoverable…", "waiver candidates", "record the
achievable baseline", "accept the current measurement as the target", any
conditional branch ending in "file a waiver".

Measurement-only antipattern: a "verify 100%" task whose success criterion is
"measure the TOTAL" is a waiver in disguise. The task must hard-gate phase
completion on per-file 100% — run `bin/flow ci`, and if below 100% return to the
test task; never "record the baseline".

Enforced by `bin/test`'s `--fail-under-lines/regions/functions 100` (pinned,
never lowered) and the Review reviewer agent flagging any added waiver file.
