//! Contract test: agent-guidance rules in `.claude/rules/` must stay terse.
//!
//! WHY THIS LIVES HERE (and not in a rule file): per
//! `.claude/rules/rule-authoring.md`, a rule exists only to steer the agent,
//! and the whole `.claude/rules/` corpus is injected into the agent's context
//! on every session start and after every compaction. Narrative bloat
//! (rationale, history, examples, "How to Apply", cross-references,
//! "Enforcement: test X asserts…") costs context on every session and changes
//! nothing about what the agent does. This test is the mechanical guard that
//! keeps the corpus terse so new rules cannot reintroduce the bloat.
//!
//! The gate, per `.claude/rules/rule-authoring.md`:
//!   (a) each `.claude/rules/*.md` file is <= MAX_LINES lines, AND
//!   (b) no file contains a banned narrative heading.
//! All violations across every rule file are collected and reported at once
//! (the failure message is the worklist), and the test fails a single time.

use std::fs;
use std::path::PathBuf;

/// Hard line cap per rule file. A rule that cannot fit its directive + trigger
/// (+ an operative checklist where the steps ARE the behavior) under this cap
/// is a signal to cut narrative, not to raise the cap.
const MAX_LINES: usize = 40;

/// Headings that only ever introduce narrative/justification/audit prose —
/// none of which steers the agent. A rule that needs one of these is carrying
/// content that belongs in this test's doc comment or nowhere.
const BANNED_HEADINGS: &[&str] = &[
    "## Why",
    "## Rationale",
    "## Cross-References",
    "## How to Apply",
];

fn rules_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".claude/rules")
}

#[test]
fn rules_are_terse() {
    let dir = rules_dir();
    let mut entries: Vec<PathBuf> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("cannot read {}: {}", dir.display(), e))
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|x| x == "md").unwrap_or(false))
        .collect();
    entries.sort();

    let mut violations: Vec<String> = Vec::new();

    for path in entries {
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("cannot read {}: {}", path.display(), e));

        let line_count = content.lines().count();
        if line_count > MAX_LINES {
            violations.push(format!(
                "{}: {} lines (cap {})",
                name, line_count, MAX_LINES
            ));
        }

        for line in content.lines() {
            let trimmed = line.trim();
            for banned in BANNED_HEADINGS {
                if trimmed == *banned || trimmed.starts_with(&format!("{} ", banned)) {
                    violations.push(format!("{}: banned heading `{}`", name, trimmed));
                }
            }
        }
    }

    assert!(
        violations.is_empty(),
        "{} terseness violations in .claude/rules/ \
         (see .claude/rules/rule-authoring.md):\n{}",
        violations.len(),
        violations.join("\n")
    );
}
