# Panic-Safe Resource Cleanup

When code acquires a resource whose "released" state is not the default —
terminal raw mode / alternate screen / mouse capture, file/advisory locks,
mutated global state (env vars, signal handlers), fds with pending side effects —
the cleanup MUST run via a `Drop`-implementing RAII guard, never inline at
scope-exit. Rust panics unwind and skip inline cleanup; only `Drop` runs on every
exit path including panic. NOT required for allocations that release naturally
(Vec/String/Box).

The guard: a named struct holding what release needs; `impl Drop` that runs the
release best-effort with errors swallowed (`let _ = ...`, since Drop can't return
them); `Option::take` so it runs at most once; placed in scope IMMEDIATELY after
acquiring, BEFORE any work that might panic. Reference: `TerminalGuard<F>` in
`src/tui_terminal.rs` (cleanup closure injected so Drop is unit-testable).

A Plan task acquiring a resource of this class must name: the resource, the
release call, the guard struct, and its in-scope placement (before the
panic-prone work). Test it: acquire, `std::panic::catch_unwind(|| panic)`, assert
released on unwind.
