# No Placeholder Anchors

Never write a placeholder file to anchor a later tool-output redirect —
forbidden regardless of destination (`/tmp/`, `.flow-states/`, anywhere) and of
how it is created (Write tool, `touch`, `echo >`). A placeholder anchor is an
empty/stub file created intending to route a later command's output into it via
a shell redirect.

It is forbidden because: shell redirection (`>`, `>>`, `<`, `tee`) is blocked
by `validate-pretool`, so the follow-up never runs and the placeholder anchors
nothing; and fixed machine-global paths race between concurrent flows. The
harness already persists large tool output — `bin/flow ci` writes
`<project_root>/.flow-states/<branch>-ci-last.log`, readable with the Read tool.

Instead: read a `bin/flow` subcommand's persisted log; for ad-hoc commands
narrow scope or use Grep; for a real persistent artifact use the Write tool
directly (one atomic action, no anchor-then-fill); for FLOW-managed artifacts
(`plan.md`, `.flow-issue-body`, `orchestrate-queue.json`) route through
`bin/flow write-rule`. If none fits, the operation is the wrong question — fix
the design upstream.
