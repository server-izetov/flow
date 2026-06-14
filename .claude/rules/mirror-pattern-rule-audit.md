# Mirror-Pattern Rule Audit

When a plan says "mirror sibling X exactly" / "matches X's pattern" / "follow
X's convention" / "copy from X" / "same pattern as X", it must also audit
whether X currently complies with every applicable rule — a literal mirror
inherits X's pre-existing rule violations as new code, which Review then flags.

Include a Mirror Audit Table near the trigger, columns: Sibling pattern
(file:function + behavior) | Applicable rules (grep `.claude/rules/` for the
pattern's keywords — normalization, byte cap, fail-open, panic) | Compliance:

- **Compliant** — copy verbatim.
- **Gap** — X violates the rule. Do NOT copy the violation: implement the new
  code rule-compliant (extract a shared helper to fix both, or fix new + file
  Tech Debt for the sibling).
- **Tension** — X follows a documented exception that contradicts the rule.
  Copy it and cite the documenting source in a code comment.

Enforcement-light: no scanner (mirror phrasings are common); the Plan-phase
audit plus the Review reviewer agent are the instruments.
