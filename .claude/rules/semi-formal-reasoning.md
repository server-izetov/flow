# Semi-Formal Reasoning

When adding or modifying a FLOW agent, decide whether it needs a Reasoning
Discipline section with the Premise → Trace → Conclude template.

Include it when the agent produces prose findings about code behavior (bugs,
risks, violations, coverage gaps), reasons about execution paths / data flow /
state transitions, or makes claims that depend on runtime behavior.

Skip it when the agent produces concrete artifacts (e.g. a code generator),
performs process/compliance analysis rather than code-semantic analysis, or
evaluates comprehension/documentation rather than behavior.

The template, per finding:

- **Premise** — state the claim, cite specific file paths and line ranges.
- **Trace** — walk the execution path step by step, verifying each step with
  Read or Grep.
- **Conclude** — confirm or refute the premise from the trace.

Findings with incomplete traces are discarded, not reported with caveats.

For a speed-critical agent (e.g. ci-fixer), scope the discipline to retry
attempts only: first attempt direct, semi-formal reasoning on attempt 2+.
