# Production-Reachable Is Testable

If a line runs when a user invokes the public interface, a test driving the same
interface can reach it. "Untestable" is never terminal — it is one of three
states, named before any action:

1. Not reachable in production at all → dead code, delete it.
2. Reachable, but the user's environment supplies something the test's does not
   → name the missing piece; that is the fixture to build.
3. The test drives a private helper while the user drives the outer entry →
   rewrite the test to invoke the public entry (subprocess or library surface).

Only after this triage does `testability-means-simplicity.md` apply.

Terminal states: covered; deleted with reason; or an explicit question to the
user naming which fixture piece is missing. "<100%, blocked" is not a report.
"Covered elsewhere" is NOT terminal either — name the specific test function +
binary AND verify the full `bin/flow ci` aggregate reads 100/100/100 (the
per-file gate sees only its own binary).

Fixture recipes for hard cases: real TTY via `libc::openpty` + `pre_exec`
(`setsid`/`TIOCSCTTY`/`dup2`); `current_dir()` Err via `pre_exec` `rmdir` of cwd;
`read_to_string` Err via `chmod 000`; spawn Err via empty `PATH`; stdin-read fail
via closing fd 0.
