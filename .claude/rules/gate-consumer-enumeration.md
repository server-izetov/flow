# Gate Consumer Enumeration

When a plan adds a new error reason / JSON field / exit-code class / `status`
value to a Rust subcommand whose stdout is parsed by skills, hooks, or other
subcommands, it is a contract change: every consumer that parses the JSON must
be updated in the same PR, or it silently drops the new reason and the gate's
intent is defeated.

In the plan's Tasks section (not Risks), write a Consumer Enumeration Table —
one row per caller, columns: Consumer (file + parsing function/step), Output
read (which subcommand's JSON), Current handling (fields read/ignored),
Required change (one of: *Add a new branch* with named behavior / *No change* /
*Exempt*). Never leave a row blank; an absent consumer is a Plan gap.

What counts as a new reason: a new `reason` value, a new top-level JSON field
consumers parse, a new exit-code class beyond 0/1/2, or a new `status` beyond
ok/error/skipped. A field no consumer parses does NOT trigger this.

Enumerate by grepping every `bin/flow <name>` invocation in `skills/`,
`.claude/skills/`, `hooks/`, `agents/`, `src/`; read 5–10 lines around each to
classify which fields it reads; add a Code task per consumer needing a branch.
