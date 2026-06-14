# Autonomous-Flow Self-Recovery

When a tool call fails during an autonomous phase (`continue: auto`), classify
before deciding to ask the user: does the error message name the fix
(mechanical), or does it pose a question you are not authorized to answer
(substantive)? Defaulting to "ask the user" on every failure defeats the
autonomous contract.

Mechanical → resolve in-flow, log via `bin/flow log "[Phase 2] Mechanical
recovery: <what> → <retry>"`, retry with the corrected input. These name their
fix: `validate-worktree-paths` `BLOCKED:` redirects (reissue at the named
canonical path); worktree-internal `.flow-states/` Write/Edit (auto-rewritten,
no action); relative-vs-absolute path rejections (retry the other form);
Read-tool failure on a `write-rule`-written path (reissue once); compound-command
rejections (split into separate Bash calls / use Read/Grep); `git diff` with file
args blocked (use Read/Grep).

Substantive → surface via `AskUserQuestion` (the autonomous block does not apply
when the question is genuinely substantive): domain ambiguity (two valid
readings, prose doesn't disambiguate), semantic decisions affecting user-visible
behavior, and user-evidence contradicting your code reading (the evidence is
ground truth).
