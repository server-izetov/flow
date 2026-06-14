# Background Task Polling

When a Bash invocation runs in the background (`run_in_background: true`),
wait for the `task-notification` system-reminder, then Read the output
file once. Never poll: repeated Reads of the output path before the
notification are forbidden — mid-stream reads are partial snapshots, and
the notification is the only signal the file is complete.

If you have other useful work while it runs, do it; otherwise output
nothing — the notification arrives on its own. Reading the output once
after completion is correct; the prohibition is only the polling loop.
