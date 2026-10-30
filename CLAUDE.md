# agentic-browser

## Start here

- **What:** a learning project in Rust: an agent-first browser engine, exposed over MCP, with
  a local-first archive of visited pages. No engine code yet; design, decisions and a threat
  model so far.
- **Main code:** `docs/adr/` (decisions), `docs/design/` (living specs),
  `docs/security/threat-model.md`, `playground/` (Rust scratch crate). The resume path and
  sequenced work are in `docs/TODO.md`.
- **Current focus:** Cycle 0 (adapt `.claude/`, add `deny.toml`), then Cycle 1 (workspace
  skeleton). The gate before Cycle 0 passed on 2026-10-30 (ADR 0010).

## Working here

- **ADRs are never rewritten.** Supersede with a new ADR, or annotate in place with a dated
  note. Decisions are numbered `D1…`, contiguous across ADRs; check the last one before adding.
- Open questions keep their prefixes: `Q` (MCP design), `S` (security design), `O` (ADRs),
  threats `T` (threat model).
- Rust is learned alongside the build (`docs/learning/rust/plan.md`). Each session gets a
  `session-NN.md`; update `status`/`session` in `concepts.csv` and fill `code_refs` only with
  real paths, once the code exists.
- Agents build in cycles (ADR 0008 D45): one spec, one branch, merged by Gabriel after review.
- **This clone is a sparse checkout** (ADR 0010 D48): `docs/`, `.claude/`, `playground/` and
  the root files. Never widen it or disable it: agent-written crates are reviewed on GitHub and
  built only in the cloud or as the separate macOS user.
- The GitHub repo is **public**. Commit when asked; push only on an explicit OK.
