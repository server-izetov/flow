# Permission-Blocked Workarounds

When the permission model (allow/deny lists + the `validate-pretool` hook)
blocks an operation, never create a new artifact as a workaround — never
write a helper script to batch operations the Bash allow list forbids. A
script the model cannot execute it also cannot delete, leaving an orphan;
and hiding the orphan tempts a second violation (editing `.gitignore`
unasked).

When you need N operations and the shell idiom is blocked:

1. Fire N Bash tool calls directly — individual allow-listed commands work
   and leave no orphan. Overhead is real but capped.
2. Or stop and ask the user (fire N calls / expand allow list for one named
   script / change approach).

Never write a `.sh` / `.py` / `.rb` "temporary" workaround during an active
flow. Temporary files without a cleanup path are not temporary.
