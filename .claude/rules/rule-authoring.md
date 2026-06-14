# Rule Authoring

These rules exist to steer your behavior — nothing else reads them.
Include only what changes what you do.

- A rule is a **directive + its trigger**. Nothing else.
- Add a checklist only when the steps *are* the behavior (the exact
  normalization, the byte cap, the banned variants).
- Cut every time: rationale/why, history, examples, "How to Apply",
  cross-references, "Enforcement: test X asserts…". Audit detail a
  maintainer needs lives in the enforcing test's doc comment.
- Hard cap: 40 lines per rule file (CI-enforced).
