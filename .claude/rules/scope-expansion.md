# Scope Expansion for Sweeping Fixes

When an issue cites N violations and the Plan-phase sweep finds many more
of the same class, expand the PR to cover the full sweep only when ALL
three hold:

1. **Fixes are inert** — text-only, cannot introduce runtime bugs
   (comment/doc rewrites, non-API renames, permission additions).
2. **One automated guard prevents regression** — a single test / scanner /
   lint / schema check covers the class forward. If no such guard can be
   written, expansion is a one-shot cleanup future PRs will undo.
3. **Splitting would re-do work** — the sweep already mapped the files;
   per-file issues force every future session to re-explore them.

If any condition fails, bound the PR to the cited violations and file a
follow-up `Tech Debt` issue with the sweep inventory (reference this PR
number). When expanding: enumerate all affected files in the plan and add
the guard task depending on all rewrite tasks, so it lands last.

Trigger: the sweep finds ≥2× the cited count → expansion is a live option.
Be honest about condition 2 — "a scanner would be nice" is not "a scanner
can detect this with an acceptable false-positive rate."
