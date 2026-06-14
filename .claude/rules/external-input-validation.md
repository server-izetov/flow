# External Input Validation

When a constructor validates input via `assert!`/`panic!`/an invariant check,
callers that source the input from outside the process (git output, CLI flag,
state file, env, parsed subprocess JSON) must use a FALLIBLE variant — a panic
downstream of unchecked external input is a DoS on legitimate inputs. A
`--branch` override is external input, no more trusted than git output.

Reference: `FlowPaths::try_new(root, branch) -> Option<Self>` (None when
`is_valid_branch` fails); `FlowPaths::is_valid_branch` for pre-validation.

- External-source callers (CLI, git output, hooks, `--branch`) → pattern-match
  the `Option`/`Result`, treat the invalid case as expected control flow ("no
  active flow" / structured error), never a panic.
- A caller holding a branch validated upstream may chain `.expect("<boundary
  naming the sanitizer>")` (documentation, not a panic vector). Structurally-
  provable carve-out: a branch from `Path::file_name()` is OS-guaranteed
  `/`-free, so `.expect` is sound.

Hooks (`src/hooks/*.rs`) and CLI subcommands accepting `--branch` default to
`try_new` + pattern-match; a panic there crashes the session/shell.

A plan adding/tightening a validation must include a caller audit: each
callsite's input source, classification (guaranteed-valid / trusted-external /
untrusted), and handling. Test every rejection class (empty, `.`, `..`, NUL).
