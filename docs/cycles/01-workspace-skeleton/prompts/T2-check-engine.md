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
The script exits 0 on the workspace as it is; non-zero, with a message naming the
violation, on each of eight deliberate violations; and still 0 on two edits that break
neither rule.
- **Evidenced by:**
  1. `bash scripts/check-engine.sh` exits 0 on the committed workspace.
  2. Eight temporary edits, each made alone, checked, and reverted before the next. Each must
     produce a non-zero exit:
     a. add a `[dependencies]` section holding `net = { path = "../net" }` to `crates/engine/Cargo.toml`;
     b. add a `[dependencies]` section holding `serde = "1"` to `crates/engine/Cargo.toml` (a dependency
        other than `net` that is not on the allowlist);
     c. add `net` to the script's allowlist **and** make edit a (`net` stays refused even
        when listed);
     d. add `use std::fs;` to `crates/engine/src/lib.rs`;
     e. add `use std::process;` to `crates/engine/src/lib.rs`;
     f. add `pub fn f() -> Option<std::net::Ipv4Addr> { None }` to
        `crates/engine/src/lib.rs` (a path, no `use`);
     g. add `pub fn g() { let _ = std::fs::read("x"); }` to `crates/engine/src/lib.rs`
        (a path to an item inside `std::fs`);
     h. add this grouped import, over four lines, to `crates/engine/src/lib.rs`:
        ```rust
        use std::{
            collections::HashMap,
            env,
        };
        ```
  3. Two temporary edits, made and reverted the same way, that must each exit **0**:
     i. add `use std::collections::HashMap;` to `crates/engine/src/lib.rs` (`std` itself
        is not banned);
     j. add a `[dev-dependencies]` section holding `serde = "1"` to `crates/engine/Cargo.toml` (rule 1
        covers normal dependencies only).
  4. After reverting, `git status --short` shows only `scripts/check-engine.sh` as new
     (`Cargo.lock` unchanged), and `bash scripts/check-engine.sh` exits 0 again.

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
  Uses `cargo metadata --no-deps --offline --format-version 1` for rule 1: it reads the
  manifests only, so it needs no network and never touches `Cargo.lock`, and it lists
  `serde = "1"` without fetching it. Prints one line per
  violation found, naming the rule and the crate or file, and exits non-zero if there is any;
  prints one line saying the check passed and exits 0 otherwise.

# Instructions
1. CREATE `scripts/check-engine.sh`.
2. Run evidence item 1.
3. FOR EACH edit in evidence items 2 and 3 (a to j): make the edit, run the script, record
   the exit code and output, revert the edit.
4. Run evidence item 4.

# Constraints
- None of the ten temporary edits is ever committed. Revert each with an edit, not with
  `git checkout` or `git stash`. Edit c changes the script itself: revert that too before
  the next edit.
- Do not edit any file except `scripts/check-engine.sh`, apart from the temporary edits.
- If the script cannot detect a violation after two attempts, stop and report which one.

# Response format
One line for the Deliverable, marked done or not done. Then evidence item 1's output; for each
edit a to j, its exit code and the script's output verbatim; then evidence item 4's output.
No other prose.
