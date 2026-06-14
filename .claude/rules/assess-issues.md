# Assess Issues by Reading Code

When asked "is this issue still relevant?", never grep the issue's own
phrases and treat matches as confirmation — that is confirmation bias.
Assess from reading what the existing code actually does, not from confirming
the issue.

1. Fetch the issue and its claims.
2. Read the full relevant sections of every referenced file.
   If the issue names no files, search the codebase for the behavior, then read it.
3. Check `gh pr list --search "<N>"` and `git log --all --grep "#<N>"`
   for already-shipped work; verify by reading the cited code, not the
   PR title.
4. Compare current code against the claims independently, then judge.

Existing code that looks like the request does NOT mean done — the issue
may target an incompleteness or gap in that code. If current behavior
differs from the ask in any dimension (scope, paths, conditions), the
issue is asking for the delta.
