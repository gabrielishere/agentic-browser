---
name: 01-T2-check-engine
description: Write scripts/check-engine.sh and show it fails on each violation
task: T2
model: claude-sonnet-5-5
---

# Objective
Write `scripts/check-engine.sh`, the command that enforces the engine's two rules: it gains no
dependency that has not been approved, and its code performs no I/O. Then prove the script
fails when either rule is broken.

# Outcome
The script exits 0 on the workspace as it is, and non-zero, with a message naming the
violation, on each of three deliberate violations.
- **Evidenced by:**
  1. `bash scripts/check-engine.sh` exits 0 on the committed workspace.
  2. Three temporary edits, each made alone, checked, and reverted before the next. Each must
     produce a non-zero exit:
     a. add `net = { path = "../net" }` under `[dependencies]` in `crates/engine/Cargo.toml`;
     b. add `use std::fs;` to `crates/engine/src/lib.rs`;
     c. add `use std::{collections::HashMap, env};` to `crates/engine/src/lib.rs`.
  3. After reverting, `git status --short` shows only `scripts/check-engine.sh` as new, and
     `bash scripts/check-engine.sh` exits 0 again.

# Task context
- The workspace exists: `crates/engine`, `crates/net`, `crates/browser`. `engine` currently
  has no dependencies.
- Rule 1 (dependencies): `engine`'s normal (non-dev) dependencies must all be on an allowlist
  kept at the top of the script. The allowlist is **empty** in this cycle. `net` must never be
  allowed, even if someone adds it to the list: check for it separately.
- Rule 2 (no I/O): no file under `crates/engine/src/` may name `std::fs`, `std::net`,
  `std::process` or `std::env`, whether written as a path (`std::fs::read`), a plain `use`
  (`use std::fs;`), or inside a grouped `use std::{…}`, including grouped imports spread
  over several lines.
- The script runs in the cloud session and in GitHub review, so it uses only `bash`, `cargo`
  and standard text tools (`grep`, `sed`, `awk`).

# Deliverables
- **CREATE** `scripts/check-engine.sh`: runnable with `bash`, from the repository root.
  Uses `cargo metadata` or `cargo tree -p engine -e normal` for rule 1. Prints one line per
  violation found, naming the rule and the crate or file, and exits non-zero if there is any;
  prints one line saying the check passed and exits 0 otherwise.

# Instructions
1. CREATE `scripts/check-engine.sh`.
2. Run evidence item 1.
3. FOR EACH violation in evidence item 2: make the edit, run the script, record the exit code
   and output, revert the edit.
4. Run evidence item 3.

# Constraints
- The three violations are never committed. Revert each with an edit, not with
  `git checkout` or `git stash`.
- Do not edit any file except `scripts/check-engine.sh`, apart from the temporary violations.
- If the script cannot detect a violation after two attempts, stop and report which one.

# Response format
One line for the Deliverable, marked done or not done. Then evidence item 1's output; for each
violation a, b and c, its exit code and the script's output verbatim; then evidence item 3's
output. No other prose.
