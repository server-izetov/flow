# Post-Compaction Recovery

On every compaction, FLOW writes the full pre-compaction analysis (in-flight
decisions, findings, classifications, rationale) to the `compact_summary`
field in `.flow-states/<branch>/state.json`.

On post-compaction resume during an active flow, when a pre-compaction detail
seems missing: Read `.flow-states/<branch>/state.json` and consult
`compact_summary` FIRST. "Lost to compaction" is not a valid conclusion until
you have read that field and confirmed the detail absent.

Never read the raw transcript JSONL (`~/.claude/projects/.../<id>.jsonl`) to
recover context — it is out-of-project (permission prompt) and
`validate-claude-paths` blocks Read/Edit/Write on the transcript root anyway.
`compact_summary` is the recovery surface.
