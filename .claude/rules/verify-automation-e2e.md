# Verify Automation End-to-End

Never ship an automation feature without running its full execution path first.
If a feature claims to "run X automatically", actually run it before marking it
complete.

The only sanctioned deferral is a genuine bootstrapping conflict: the test would
re-enter the active workflow (a FLOW lifecycle feature can't be E2E-tested from
inside an active flow) OR requires interactive credential/hardware/UI approval
the agent cannot provide. "Inconvenient" or "slow" does not qualify.

A sanctioned deferral must satisfy ALL of:

1. Logged via `bin/flow log` naming what was deferred, why bootstrapping
   prevented in-session testing, and the exact command the user runs post-merge
   to verify.
2. Documented in the commit message body (same explanation).
3. NOT used for security-sensitive features (auth, authz, crypto, secrets,
   sandbox escapes, external-input validation) — redesign so the path is
   exercised inline instead.

A TODO comment, a PR note without a log entry, or a verbal commitment do NOT
count as flagging.
