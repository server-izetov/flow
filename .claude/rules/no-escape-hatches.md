# No Escape Hatches

Use sanctioned tools; never route around them. The model's action surface is
Read/Edit/Write/Glob/Grep + the Bash allow-list. Any construct hiding the actual
command from that surface is forbidden, even when the underlying operation is
legitimate.

Forbidden shapes → sanctioned alternative:

- Shell-eval (`bash -c '<cmd>'`, `sh -c`, `zsh -c`, `eval`) → separate Bash calls
  per command.
- Interpreter-eval (`python -c`, `perl -e/-E`, `ruby -e`, `node -e/-p`,
  `osascript -e`, `lua -e`) → Read/Write tools; a real script + `bin/*` runners.
- Command-wrapper (`xargs <cmd>`, `rtk proxy <cmd>`) → separate Bash calls /
  invoke the command directly.
- Wrapper-launcher (`env`/`time`/`nice`/`nohup`/`taskset`/`ionice <cmd>`) →
  invoke the inner program directly.
- Network-bridge (`nc`, direct `ssh`) and inter-process (`tmux send-keys`,
  `screen -X`) → the dedicated network surface / approved ssh wrapper.
- Bypass-shortcut: writing `_continue_pending=commit` then calling
  `bin/flow finalize-commit` directly → invoke `/flow:flow-commit`.

Indirect forms (absolute paths, env-var prefixes, combined short flags like
`bash -lc`, wrapper nesting) count the same. Legitimate non-eval uses pass
(`bash -n script.sh`, `ssh-keygen`, `tmux ls`, `rtk discover`).

Enforced by `FLOW_DENY` (direct forms), the structural escape-hatch layer in
`validate_pretool` (indirect/wrapper forms, settings-independent), and the Layer
10 commit gate's transcript-walker carve-outs (bypass shortcuts).
