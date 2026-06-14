# Security Gates

A CLI/entry-point gate guarding an action against caller input (phase, outcome,
path, flag) must be robust to input variation and fail-closed on uncertainty.

- **Normalize before comparing.** Every gate-deciding string: strip NULs
  (`.replace('\0', "")`), `.trim()`, and `.to_ascii_lowercase()` when case-
  insensitive. Normalize BOTH sides. Extract a shared `normalize_gate_input`.
- **Positive allowlist, not denylist.** Encode "only X permitted" as allowlist
  membership over normalized input, never a denylist of forbidden values (a new
  domain value silently passes a denylist).
- **Fail closed when state is unreliable.** Reading `current_phase` etc. from a
  state file: no file / empty → pass (not an active flow); parses with the field
  → apply gate; non-empty but unparseable / wrong root type / missing field →
  REJECT (a corrupt state file is not an escape hatch).
- **Gate-action atomicity.** When a gate validates by TRANSFORMING input
  (resolving a relative path against project_root, normalizing case), the action
  after the gate must consume the TRANSFORMED value, never the raw input — else
  `fs::write(args.path, …)` re-resolves against process cwd and lands where the
  gate would have rejected. Bind the transformed value; pass it downstream.

Enumerate bypass variants in the plan BEFORE coding, one test each: whitespace,
case, embedded NUL, type variants (number/bool/null/missing/array), BOM,
duplicate keys, empty/single-char, override flag set/unset/`=false`. Add a
binary-level integration test spawning the real CLI with prepared state.
