# ADR 0010 — Agent-written code never runs on the author's account

- **Status:** Accepted
- **Date:** 2026-10-30
- **Deciders:** Gabriel
- **Amends:** [ADR 0009](0009-builder-agents-run-in-the-cloud.md) D47 (where code is read);
  [ADR 0005](0005-sandboxing-boundary-and-io-free-engine-core.md) D32 step 1 (when it is needed)
- **Answers:** [`design/security.md`](../design/security.md) S3 and S9

---

## Context

D47 says a cycle branch is read before it runs here. That assumes reading and
running are separate acts. On this machine they are not: opening a Rust
workspace in an editor starts rust-analyzer, which runs every build script and
proc-macro it finds. Checking out a cycle branch to read it is enough to run it.

D47 also left S3 open: how a new dependency is approved, and where that is
recorded. Until S3 was answered, no branch that adds a dependency could be built.

ADR 0009's test applies again: **prefer a boundary that removes an ingredient by
construction to one that removes it by configuration.** A rule to "not open it
in the editor" is configuration. Not having the code on disk is construction.

---

## Decisions

### D48 — Agent-written code never lands on the author's main account

- The local clone is a **sparse checkout** (cone mode): `docs/`, `.claude/`,
  `playground/` and the root files. Crates written by builder agents are not
  checked out.
- Cycle branches are **reviewed on GitHub**, which displays code without
  running it. Builds and tests run in the cloud session (D46).
- Agent-written code runs on this machine **only as the separate macOS user**
  (D32 step 1), which clones and builds it there.
- `playground/` is the author's own code and stays local.

This replaces D47's mechanism. Its intent, that code is read before it reaches
the author's files, is now true by construction.

**Consequence for D32 step 1:** it was due before the engine meets real websites
(rung 4). Rung 1 paints to a window the author will want to see, so the
separate user is now needed **before rung 1 is first run locally**. Cycles 0
and 1 (the `.claude/` adaptation and the workspace skeleton) do not need it.

### D49 — Dependencies are approved in the spec and enforced by `cargo-deny`

- **Approval:** a cycle spec's `Constraints` names every direct dependency the
  cycle may add. A builder agent adds no crate the spec does not name.
  Approving the spec approves the crate.
- **Enforcement:** Cycle 0 adds a `deny.toml`. Sources are restricted to
  crates.io (no git dependencies, no other registries), and advisories are
  checked. It runs in the cloud session alongside the tests.
- **Record:** `Cargo.lock` is committed. Its diff is part of the pull request
  and is read with it.
- **Transitive crates are not allowlisted.** `html5ever` and `wgpu` alone bring
  in hundreds, and a list that long would be approved without being read (R10).
  Under D48 they run only in the cloud and as the separate user, so they are
  contained by where they run, not by review.

### D50 — A gate precedes Cycle 0, and passes with this ADR

The gate: D46 set up, D48 decided, S3 answered (D49). All three hold as of this
ADR, so **Cycle 0 is unblocked.**

The D46 merge gap (an agent, acting as the author, could merge its own pull
request) does not block the gate. Under D48 a self-merged change sits in a
public repository and runs nowhere on this machine. It is accepted as R11.

---

## Rejected

- **Keep D47 and review locally with care.** It depends on remembering not to
  open the editor. Configuration, not construction.
- **Allowlist every crate, transitive ones included, in `deny.toml`.** Too long
  to read honestly. It would fail as R10 describes.
- **Spec `Constraints` only, no `cargo-deny`.** It approves crates but enforces
  nothing: a git dependency or an advisory would get through unseen.

---

## Risks

### R11 — The D46 merge gap stays open

An agent acting as the author can merge its own pull request, so "merged by
Gabriel" (D45) is agreed, not enforced. Accepted because under D48 a bad merge
reaches only the public repository. **Revisit when** builder agents can get a
separate, lower-permission identity, or before anything built from `main` runs
outside the separate user.

### R12 — The sparse checkout is widened by accident

`git sparse-checkout disable`, or a new path added to the cone, brings agent
code back onto this account, and the editor runs it on open. **Mitigation:**
the sparse paths are written down here and in `CLAUDE.md`, so widening them is
a visible departure from a rule.

---

## Consequences

- S3 is answered by D49 and S9 by D50. S2 is unchanged.
- D32 step 1 moves from "before rung 4" to "before rung 1 is first run locally".
- Cycle 0 gains a task: add `deny.toml`.
- The author's learning sessions (ADR 0008) read code on GitHub, or as the
  separate user, not in the main account's editor.
