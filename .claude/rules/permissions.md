# Permission Patterns

- **Specificity over breadth.** Use the narrowest pattern that serves a known
  consumer — `Read(//tmp/*.md)` not `Read(//tmp/*)`. Directory wildcards only when
  every file is a valid target.
- **Consumer traceability.** Every allow-list pattern must name a specific
  skill/hook/tool that needs it (in the commit message). No speculative patterns.
- `/tmp/` allows are symmetric R+W over a closed extension set (`.txt .diff .patch
  .md .json .jsonl`); for other extensions prefer `.flow-states/<branch>/`.

When a skill adds a new bash command, the plan must confirm its first token matches
an existing `UNIVERSAL_ALLOW` entry, OR add the `Bash(<pattern>)` to BOTH
`src/prime_check.rs::UNIVERSAL_ALLOW` AND `skills/flow-prime/SKILL.md` (bump
`CURRENT_CONFIG_HASH`). Prefer routing through `bin/flow` (covered by `Bash(*bin/flow
*)`) over a new `bin/*` wildcard. New `FLOW_DENY` glob patterns must be hand-compiled
and bypass-enumerated before adding (the literal-space slot is required input; prefer
a structural check in `validate_pretool` for wide surfaces).

Never remove an existing `.claude/settings.json` entry without explicit user ask
(prime-time `merge_settings` "allow wins" is the one sanctioned exception). Never
edit permissions mid-flow inside a worktree (Claude Code enforces immediately).

Shared config files (`.gitignore`, `.gitattributes`, `Makefile`/`Rakefile`/
`justfile`, `package.json`, `requirements.txt`, `go.mod`, `Cargo.toml`, anything
under `.github/` or `.config/`) must NOT be edited during an active flow without
explicit user permission — they affect every engineer. `validate-worktree-paths`
blocks such edits; recovery is two-half and model-unforgeable: the USER replies with
the exact line `approve shared-config: <path>`, then the model runs `bin/flow
approve-shared-config --path <path>` (self-gates on that user turn, writes a
single-use marker the gate consumes). The model never fires an AskUserQuestion for
this; on `not_user_approved`, keep waiting for the user.
