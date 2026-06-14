# Transcript Walker Cap Selection

Every production read of the persisted transcript JSONL routes through one of
three public wrappers in `src/hooks/transcript_walker.rs`, chosen by lookback +
invocation rate:

- `read_full(path)` — uncapped. For phase-boundary walkers (rare: per-agent-
  return / phase-finalize / cleanup) whose marker can sit arbitrarily far back.
  NEVER for a per-turn walker.
- `read_recency_window(path)` — `TRANSCRIPT_BYTE_CAP` (50 MB tail). For per-turn
  recency walkers (every Skill/AskUserQuestion/Stop) needing the last ~10k turns.
- `read_recent_turns(path)` — `SHARED_CONFIG_BLOCK_BYTE_CAP` (4 MB tail). For the
  blocked-AskUserQuestion hot path needing only the latest tool-call + result.

The private `read_capped(path, cap)` is the shared primitive; production code
outside the three named wrappers MUST NOT call it. Choosing the wrong cap is
silent: a phase-boundary walker on a tail cap misses far-back markers; a per-turn
walker on `read_full` blows the hot path. Enforced by
`read_capped_only_called_inside_named_helpers` in
`tests/hooks/transcript_walker.rs`.
