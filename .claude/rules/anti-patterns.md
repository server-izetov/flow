# Anti-Patterns

- **Inline Output.** Render review-output (plan, DAG, findings) as
  formatted text in the response. Never tell the user to "look at" or
  "take a look at" a file — they cannot see file contents otherwise.
- **Fix before remove.** When a feature is broken, fix it first. Only
  propose removal after showing it cannot be fixed, or when the user asks.
- **Never offer to skip workflow steps.** When a hook blocks or the user
  is frustrated with a step, never offer skipping it. Fix the blocked
  action and retry; if unfixable, report why and wait for direction.
- **Precise rule-file mechanism descriptions.** In a rule's enforcement
  prose, name the exact match mechanism — "starts with" = prefix, "ends
  with" = suffix, "contains" = substring, "first token" = tokenization.
  Define project vocabulary on first use or cite its defining file.
