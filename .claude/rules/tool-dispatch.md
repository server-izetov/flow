# Tool Dispatch

`bin/flow ci` (and `--format`/`--lint`/`--build`/`--test`) delegate to repo-local
`./bin/{format,lint,build,test}`. Invariants:

- **Empty tool list is a failure, not a skip.** When no executable
  `bin/{format,lint,build,test}` exist in cwd, the runner returns
  `{"status":"error"}` exit 1 pointing at `/flow:flow-prime` — never "ok
  skipped". The guard lives in BOTH `ci::run_once` and `ci::run_with_retry`; keep
  them in sync, with a test at each callsite.
- **Stub marker + sentinel suppression.** Every `assets/bin-stubs/*.sh` contains
  the literal `# FLOW-STUB-UNCONFIGURED`. `ci.rs::any_tool_is_stub` scans for it;
  when present, CI reports `status:ok stubs_detected:true` and refuses to write
  the sentinel (so the stderr reminder surfaces every run). A new stub must carry
  the marker in its source; a new dispatcher must scan before writing a sentinel.
- **`bin/test` sweeps `*.profraw` recursively under `target/llvm-cov-target/`
  (plus root `default_*.profraw`) at the top of EVERY run**, before mode
  dispatch — keeps coverage scoped to this run's profdata on the long-lived
  base-branch target. `bin/flow ci --clean` is the deep reset.
- A new stub/auto-installed script needs a full-lifecycle test (prime → marker
  present → runner ok+stubs_detected, no sentinel → reminder persists → user
  removes marker → sentinel written → respected). Every `EXCLUDE_ENTRIES` /
  `UNIVERSAL_ALLOW` / `FLOW_DENY` change bumps `compute_config_hash`; ≥3 hash
  bumps in one PR signals the design wasn't enumerated upfront.
