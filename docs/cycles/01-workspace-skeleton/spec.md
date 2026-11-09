---
spec: 01-workspace-skeleton
---

# Objective
Lay down the Cargo workspace the browser is built in: the `engine`, `net` and `browser`
crates of ADR 0007 D42, with the engine's no-I/O rule (ADR 0005 D30) checked by a command
rather than by memory, and dependency policy (ADR 0010 D49, ADR 0011 D53) running from the
first build.

# Outcomes
- **O1** From a fresh clone, `cargo build --workspace` and `cargo test --workspace` succeed,
  and the workspace has exactly three members: `engine` (library), `net` (library) and
  `browser` (binary). Each library has at least one passing test.
- **O2** `cargo run -p browser` runs, prints one line naming the program, and exits 0.
  `browser` depends on `engine` and `net`; `engine` depends on neither.
- **O3** `scripts/check-engine.sh` exits 0 on the workspace as built, and exits non-zero
  when `engine` gains any normal dependency not on the script's allowlist (empty in this
  cycle; `net` is never allowed), or when a source file under `crates/engine/src/` names
  `std::fs`, `std::net`, `std::process` or `std::env`, including inside a grouped
  `use std::{…}`. A check that cannot fail is not a check: the failing cases are shown.
- **O4** `cargo deny check` passes on the workspace with the repository's `deny.toml`
  unchanged, and `Cargo.lock` is committed.

# Constraints
- **Rust edition 2024**, workspace resolver 3. Every crate carries
  `license = "MIT OR Apache-2.0"` and `publish = false`, set once in
  `[workspace.package]` and inherited.
- **Layout:** members under `crates/` (`crates/engine`, `crates/net`, `crates/browser`).
  The root `Cargo.toml` is a virtual manifest: a workspace with no package of its own.
- **Dependencies:** none from crates.io. The only dependencies are path dependencies
  between the three members (`browser` → `engine`, `browser` → `net`). No dev-dependencies.
- **`engine` code:** no `std::fs`, `std::net`, `std::process`, `std::env` (D30). Tests in
  `engine` take their input as literals; they do not read files.
- **`deny.toml` is not edited.** If `cargo deny check` fails, the task stops and reports.
- **Commands this run may use:** `cargo build`, `cargo test`, `cargo run`, `cargo tree`,
  `cargo metadata`, `cargo install --locked cargo-deny`, `cargo deny check`,
  `bash scripts/check-engine.sh`, and `git` for staging and committing. Network: crates.io
  only (ADR 0011 D52); nothing is fetched from anywhere else except by `cargo deny check`'s
  own advisory database (see T3).
- **No code beyond the skeleton.** No DOM, parser, window or networking code: each crate's
  library is a placeholder that its one test exercises.

# Shared
- The root `Cargo.toml` and the three crates — created by **T1**; T2 and T3 read them.

# Tasks

## T1 — Workspace and the three crates
**Objective:** Create the workspace and its three member crates, with one test in each
library and a `browser` binary that runs.
**Outcome:** The workspace builds, tests pass, and the browser runs. → serves **O1**, **O2**
**Reads:** `deny.toml` (for the licence the crates must carry).
**Deliverables:**
- CREATE `Cargo.toml`
- CREATE `crates/engine/Cargo.toml`
- CREATE `crates/engine/src/lib.rs`
- CREATE `crates/net/Cargo.toml`
- CREATE `crates/net/src/lib.rs`
- CREATE `crates/browser/Cargo.toml`
- CREATE `crates/browser/src/main.rs`
- CREATE `Cargo.lock`

**Evidenced by:** `cargo build --workspace`, `cargo test --workspace` and
`cargo run -p browser`, each exiting 0; `cargo tree -p browser` showing `engine` and `net`
under it, and `cargo tree -p engine` showing `engine` alone.

## T2 — The engine check
**Objective:** Write the script that fails when the engine gains a dependency or an I/O
import, and show that it fails when it should.
**Outcome:** D30 is enforced by a command anyone can run. → serves **O3**
**Reads:** the workspace from T1.
**Deliverables:**
- CREATE `scripts/check-engine.sh`

**Evidenced by:** `bash scripts/check-engine.sh` exiting 0 on the workspace; then, on
temporary edits that are reverted afterwards and never committed, a non-zero exit for each
of: `engine` depending on `net`; a `use std::fs;` line in `crates/engine/src/lib.rs`; a
`use std::{collections::HashMap, env};` line there. Report each exit code and the
script's message.

## T3 — Dependency policy runs
**Objective:** Install `cargo-deny` and run it against the workspace.
**Outcome:** The dependency policy runs from the first build. → serves **O4**
**Reads:** `deny.toml`, the workspace and `Cargo.lock` from T1.
**Deliverables:**
- none: this task produces evidence, not files.

**Evidenced by:** `cargo deny check` exiting 0. If it cannot fetch its advisory database
(it is downloaded from GitHub, `rustsec/advisory-db`, which ADR 0011 records as unverified
behind the GitHub proxy), report the error verbatim and stop. Do not work around it, and do
not edit `deny.toml`.
