# Skill-Name Form Variance in Discriminators

When a hook predicate or transcript walker compares a transcript-recorded
`input.skill` against a literal to make a gate decision, and the skill can be
recorded under more than one form, accept every form — never hardcode one.
(`input.skill` is recorded verbatim; a plugin skill appears as both bare
`<name>` and namespaced `<plugin>:<name>`. These predicates fail open, so a
single-form match silently never fires.)

Route the comparison through a named helper `is_<skill>_skill(s) -> bool` that:

1. Normalizes via `normalize_gate_input` (NUL strip + trim + ASCII-lowercase).
2. Returns true for every valid form (e.g. `n == "decompose" || n ==
   "decompose:decompose"`) using full-string `==` — never `contains` /
   `starts_with`, so a superstring cannot match.

Reference: `is_decompose_skill` in `src/hooks/stop_continue.rs`.

Single-form matches ARE correct for skills recorded under only one form
(user-only `flow-release` is bare; `flow:flow-commit` is namespaced). The
test: can the model produce this skill name in more than one shape? Yes →
helper accepting all forms; no → single-literal match.
