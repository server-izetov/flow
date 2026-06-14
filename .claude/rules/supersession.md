# Supersession

When a PR makes other code permanently redundant, delete that code in the same
PR — not as follow-up, not tech debt. Test: if deleting the code leaves the PR's
behavior unchanged, it is superseded.

Shapes: an authoritative replacement of broken/best-effort code; a deterministic
guard that makes downstream defensive handling unreachable; a unified handler
replacing specialized paths; a deprecated API once the switchover lands; a
removed producer whose output (breadcrumb file, state field, emitted event) is
read by a now-orphaned consumer.

Plan phase: enumerate superseded code in Exploration and add deletion tasks. When
the PR's primary action IS a deletion, run the inverse (cascading) analysis —
trace every consumer of each deleted output, classify as has-surviving-producer
(keep) / orphan (delete in same commit) / partially-orphan (coverage-gap risk).

Review phase: apply the supersession test to every real finding BEFORE the
Real/False-positive classification — if deleting the described code leaves
behavior unchanged, route to deletion regardless of which file it lives in (even
files outside the PR diff).
