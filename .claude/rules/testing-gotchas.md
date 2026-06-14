# Testing Gotchas

- **Fixtures: no symlinks to real binaries** — writes follow the link and
  overwrite the target. Use wrapper scripts (`exec <real> "$@"`).
- **Host-env leaks** — set `current_dir` to the fixture repo when the code under
  test calls `current_branch()`/`project_root()`/any git subprocess.
- **Rust parallel env races** — never `set_var`/`remove_var` in tests; extract a
  pure fn taking the values as params and test that.
- **Failure after a change** — question the change first, not the test. Update
  the test only after confirming the change is correct.
- **Cross-branch baseline before "infra bug"** — run the same command on the
  integration branch first; default attribution is "my branch broke it".
- **Ambiguous name filters** — use the most specific distinguishing substring.
- **Subprocess-repopulated dirs** — assert the specific stale file's absence, not
  directory existence (the subprocess may recreate the dir).
- **Doc comment must support the test name** — never disavow the assertion.
- **Message-content assertions per variant** — assert message content for each
  variant, not just `is_some()`.
- **Suffix-match coverage** — test both bare (`bin/ci`) and absolute-path forms.
- **Subsection-local assertions** — bound the slice with `split_once(start)` then
  `split_once(end_marker)` (`unwrap_or(tail)`); never `contains` over the whole file.
- **macOS subprocess paths** — canonicalize the tempdir root once
  (`dir.path().canonicalize()`) so child cwd and file_path share resolution.
- **Document fixture helpers** — returns, each param's meaning, non-obvious
  production invariants.
- **Timing-sensitive tests** — inject a time/sleep seam (`filetime`, `now_fn`),
  never real `thread::sleep`.
