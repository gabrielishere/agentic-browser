# ADR 0011 — What builder agents may reach

- **Status:** Accepted
- **Date:** 2026-11-09
- **Deciders:** Gabriel
- **Answers:** [`design/security.md`](../design/security.md) S2
- **Affects:** every cycle spec

---

## Context

S2 asked what builder agents may execute, and whether they get network access beyond
`cargo fetch`. ADR 0009 put them in cloud sessions: an Anthropic-managed VM holding a clone
of this public repository and nothing else, with git credentials kept outside it by a proxy.

Checked against the cloud-environments documentation on 2026-11-09:

- Rust (`rustc`, `cargo`) is preinstalled in the VM.
- GitHub traffic goes through its own proxy and **bypasses the network allowlist** at every
  level, including None. The proxy does not limit which branches a push updates; `main`'s
  branch protection does that.
- A **Custom** level takes an explicit domain list, with the default list optional.

"Agent" here always means a **builder agent** (orchestrator, implementor), never a consumer
agent. A **cycle session** is one cloud session running one spec.

## Decisions

### D51 — The VM is the boundary; commands are named per spec

A builder agent may run and write anything **inside the VM**. Only the cycle branch leaves
it. There is no global command allowlist: the VM limits what an agent can *reach*, and with
nothing private in reach, limiting what it *runs* adds rules without adding protection. Each
spec's `Constraints` names the commands its tasks run, so the orchestrator can gate them and
the author can see them.

### D52 — A cycle session reaches crates.io and nothing else

The cloud environment uses **Custom** network access with exactly `crates.io`,
`index.crates.io` and `static.crates.io`, and the default list unticked. GitHub is reached
only through the proxy. A domain added later is added by a dated note here, not silently.

### D53 — `deny.toml` also checks licences

Beyond D49 (sources, advisories), every crate's licence must be on a permissive allowlist
(MIT, Apache-2.0, BSD and similar), matching this repository's own MIT/Apache-2.0. Bans wait
until there are crates to ban.

## Alternatives

- **A global command allowlist** — protects little behind a VM that holds only public code,
  and has to be widened every cycle.
- **Trusted network level** — reaches dozens of registries and cloud SDK domains no cycle
  needs.

## Consequences

- The author sets the environment's network level once, on claude.ai, before Cycle 1.
- `cargo-deny` is installed in the session (`cargo install --locked cargo-deny`, from
  crates.io).
- **Unverified:** `cargo deny check` fetches its advisory database from GitHub
  (`rustsec/advisory-db`). Whether the proxy serves a repository not attached to the session
  is not documented. Cycle 1's first session shows it; if it fails, that is a dated note here.
- R9 and the D46 merge gap are unchanged.
