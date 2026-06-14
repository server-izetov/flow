# Cognitive Isolation

Run debiased analysis of in-session work in FOREGROUND sub-agents that receive ONLY
persisted artifacts (diff file, plan, rules) — never conversation history. The
parent stays alive to receive results and continue (never force a session break —
Claude Code has no auto-resume). Sub-agents are read-only (Read/Glob/Grep/Bash); the
global PreToolUse hook enforces Bash restrictions — no frontmatter hooks.

Two-tier context: context-rich (reviewer — full diff + plan + CLAUDE.md + rules
inline) vs context-sparse (pre-mortem, adversarial, documentation — the substantive
diff as a file path; they investigate standards themselves). Aim ≤250–300% context
utilization; beyond ~300% autocompact-thrash dominates.

Truncation: high-investigation agents end with a literal `## END-OF-FINDINGS` marker
(contract-tested per agent). Marker ABSENT = truncated → re-invoke with a narrower
partition (split-by-file-family / -finding-type / -phase), combine findings.
Evaluate read-overflow ("prompt is too long" + zero findings) BEFORE truncation —
its remedy is a bounded re-read (per-family diff slice), not partition.

Never supplement agent work from the parent: on agent malfunction the only two
responses are re-invoke (narrower) or surface to the user — NEVER read the inputs
yourself and record the analysis as the agent's. A marker over hollow findings is
treated as truncation.

Gating sub-agents run foreground and on-turn: when control flow branches on an agent
verdict (Review, Learn, the flow-plan plan-reviewer), invoke a FRESH foreground
`Agent` call; never `run_in_background`, never `SendMessage`-resume a completed agent
(runs in background, decouples the verdict), and never proceed past the gate while
the verdict is pending — a Stop-hook idle refusal while waiting IS the signal the
gate went off-turn; re-issue it foreground.

Required agents register in `src/required_agents.rs::REQUIRED_AGENTS`; the
PreToolUse:Agent recorder writes `agents_returned` so `phase-finalize` can gate.
