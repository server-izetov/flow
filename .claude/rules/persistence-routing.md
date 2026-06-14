# Persistence Routing

Rules are the default. Memory is the exception. CLAUDE.md is the smallest of the
three. Apply tests in order; first match wins:

- **Imperative** ("do X" / "never Y" / "when X, do Y") → Rule
  (`.claude/rules/<topic>.md` via `bin/flow write-rule`).
- **Obey-vs-describe** — the obey-vs-describe test: does the model OBEY this every
  session (e.g. "timestamps via `now()`")? → CLAUDE.md pointer line. Or DESCRIBE how
  something
  works → a module doc comment, the `docs/` subtree, or discard.
- **Forward-applicability** — a future engineer on this project needs it → Rule.
- **User-specific** — about THIS user (role, style, context) → Memory.
- **Discoverability** — derivable from code/CLAUDE.md/rules → don't save it.

Most common error: treating "the user said never do X" as memory. The phrasing
isn't the signal — the audience is. A correction any engineer needs is a Rule.
When in doubt, write the rule. A project-local rule that MANDATES descriptive
CLAUDE.md prose is itself the misclassification — route it to a feature rule file
plus a one-line index entry.

## What CLAUDE.md Is For

Only two shapes: behavioral instructions the model obeys, and one-line pointer
indexes to rule files. It's loaded every session — every byte costs budget.

## What CLAUDE.md Is Not For

Descriptive how-it-works prose. Route it to one of three destinations:

- **Module doc comment** in `src/<name>.rs` — code mechanics found via grep/rustdoc.
- **`docs/` subtree** — long-form architecture/schema, loaded on demand.
- **Discard** — the next session can derive it from code or existing rules.
