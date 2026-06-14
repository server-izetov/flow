# Transcript Shape

Claude Code's persisted transcript JSONL carries user-typed turns AND synthetic
system turns under the same `type:"user"`. A walker finding the most recent REAL
user turn MUST call `crate::hooks::transcript_walker::is_real_user_turn` and
`continue` past synthetic turns — never stop at any user turn or filter on array
content alone.

Three synthetic shapes (all `type:"user"`):

- Tool-result wrapper — `content` is an array of `tool_result` blocks.
- Hook-injected feedback — string `content` with `isMeta:true` (Stop refusals,
  PreToolUse rejections).
- Compaction continuation — string `content`, NO `isMeta`,
  `isCompactSummary:true` (the trap an isMeta-only filter misses).

A real user turn has string `content`, no `isMeta:true`, AND no
`isCompactSummary:true` — all three checks required; none suffices alone.

Real turns split further: conversational prose (a halt trigger) vs imperative
slash-command shape (`<command-name>/<skill>` or two-line `<command-message>`),
which is user-direction not conversation — only
`most_recent_user_message_since_skill_action` distinguishes them, and watermarks
preceding prose to `None` on a `/flow:flow-continue` turn.

Mechanical contract: every user-boundary walker uses the shared predicates
(`is_real_user_turn`, `is_meta_marker_present`, `is_compact_summary_turn`) — two
patterns: helper-skip (`if !is_real_user_turn { continue }`) for real-turn
seekers, or targeted-skip (still skipping both string-content synthetic shapes)
for walkers that legitimately consume array-content turns. Inlining the
discrimination is forbidden.
