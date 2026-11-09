---
name: 01-T3-cargo-deny
description: Install cargo-deny and run it against the workspace
task: T3
model: claude-sonnet-5-5
---

# Objective
Install `cargo-deny` and run the repository's dependency policy against the new workspace,
so that policy runs from the first build.

# Outcome
`cargo deny check` passes on the workspace with `deny.toml` unchanged.
- **Evidenced by:** `cargo deny check` exits 0, and its output shows the four checks
  (advisories, bans, licenses, sources) each reported as ok.

# Task context
- `deny.toml` is at the repository root. It allows crates.io as the only source and a list of
  permissive licences. The workspace crates carry `MIT OR Apache-2.0`.
- The session's network reaches crates.io only. GitHub is reached through a proxy.
  `cargo deny check` downloads its advisory database from GitHub (`rustsec/advisory-db`), and
  whether the proxy allows that is not known: this task is where it is found out.

# Instructions
1. Run `cargo install --locked cargo-deny`.
2. Run `cargo deny check`.

# Constraints
- Do not edit `deny.toml` or any file in the workspace. This task writes nothing.
- If the advisory database cannot be fetched, report the error verbatim and stop. Do not
  disable the advisories check, point it elsewhere, or retry with other options.
- If any other check fails, report its output verbatim and stop.

# Response format
The `cargo install` result line, then the full output of `cargo deny check` and its exit
code. No other prose.
