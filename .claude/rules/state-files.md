# State Files

- **Edit safety.** Never `replace_all=True` on JSON state edits when the
  `old_string` appears in multiple contexts ("pending" is both task and phase
  status). Use targeted `old_string` with unique surrounding context.
- **Numeric fields.** Store counters (`cumulative_seconds`, `visit_count`) as
  raw integers, never formatted strings; the human format (`"<1m"`) is
  display-only. Writers produce integers; readers tolerate string/float legacy
  values via `tolerant_i64()`.
- **Corruption resilience.** Every state-reading function handles malformed
  files gracefully:
  - Empty (0 bytes) → parse error / `Err`; do not write (may be mid-creation).
  - Non-JSON → `Err`, leave unchanged.
  - Wrong root type → guard `if !(state.is_object() || state.is_null())`;
    reset wrong-type nested keys (e.g. `state["phases"]`) to `{}`.
  - Missing/wrong-type nested fields → read via `get()`+`and_then()`, not
    `IndexMut`; `tolerant_i64()` for counters.
  - A new state-touching function needs edge tests: missing file, empty file,
    wrong-type accessed fields.
