# Subprocess Argument Escaping

When a value from outside the process (state file, git/subprocess output, config,
env, CLI arg) is interpolated into a string another interpreter parses —
AppleScript (`osascript`), shell (`bash -c`), SQL, regex, JSON, GitHub Markdown
table cells — it MUST be escaped per that interpreter's literal syntax before
interpolation. Raw `format!` interpolation is an injection vector.

The escape helper: named after the target language (`escape_applescript_string`,
`escape_markdown_cell` — never generic "sanitize"); doc-comment naming the
structural characters (AppleScript: `\` and `"`; Markdown cell: `|` `\` `\n`
`\r`); exhaustively unit-tested (empty, safe-only, structural-only, mixed,
escape-char-itself); and the ONLY path to the interpolation site (no
"trusted caller" bypass).

Preferred avoidances: pass args via `.arg()` not `bash -c`; parameterized SQL
not `format!`; `serde_json::to_string` not hand-built JSON; `regex::escape`.

Write the escape helper + its tests FIRST, then the interpolation site. Add an
adversarial test: a value that would inject if raw, asserting the injected
substring cannot appear in an executable position of the output.
