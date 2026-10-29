# ADR 0009 — Builder agents run in the cloud; code is read before it runs here

- **Status:** Accepted
- **Date:** 2026-10-27
- **Deciders:** Gabriel
- **Builds on:** [ADR 0008](0008-agents-build-author-specifies-and-reviews.md) (agents build in cycles)
- **Amends:** [ADR 0005](0005-sandboxing-boundary-and-io-free-engine-core.md) D32 (which boundary covers the builder agents)
- **Answers:** [`design/security.md`](../design/security.md) S1 for builder agents; S8 in part

---

## Context

ADR 0008 has builder agents write the code. As `.claude/agents/` configures
them, they run `Bash` on the author's account, which gives them all three of
Meta's Agents Rule of Two ingredients at once: untrustworthy input (crate docs,
READMEs, compiler output), access to private data (the home directory, keys,
`personal/`), and the ability to change state and communicate (files, `cargo`,
the network). With all three, the rule says an agent should not run
unattended. This is threat T12, domain A in the threat model.

D32 step 1, a separate macOS user, was designed for the browser (domain B).
S1 asked whether it should host the builder agents too.

The test applied here: **prefer a boundary that removes an ingredient by
construction to one that removes it by configuration.**

| Option | Private data | Removed by |
|---|---|---|
| Claude Code's local sandbox | On the same machine, behind a profile | Configuration |
| Separate macOS user | On the same machine, behind file permissions | Configuration |
| **Cloud sessions** (Claude Code on the web) | **Not on the machine at all** | **Construction** |

A cloud session runs in an isolated, Anthropic-managed VM. It clones the repo
from GitHub, and the repository is public, so a fully hijacked builder agent
can take nothing that is not already published. Git credentials stay outside
the VM: a proxy attaches them to git requests server-side.

D32 step 3 rejected a container or VM for routine development because of this
machine's 8 GB. A cloud VM costs the laptop nothing, so that objection does not
apply.

---

## Decisions

### D46 — Builder agents run in cloud sessions, not on this machine

- Cycles are dispatched as cloud sessions (`claude --cloud`, or claude.ai/code).
  `.claude/agents/` loads there as it does locally.
- GitHub access is through the **Claude GitHub App, installed on this
  repository only**. Not `/web-setup`, which hands over a `gh` token that
  reaches every repository the author can.
- The VM is scratch. GitHub is the meeting point; the local clone under
  `~/gabriel/projects/` stays the real home. Work that is not pushed does not
  survive the VM.
- Builder agents push to the cycle branch only (D45). `main` is protected on
  GitHub so that this is enforced, not agreed.

> **2026-10-29, set up and smoke-tested:** a cloud session pushes as the
> author, not as a separate identity. With admins exempt from the `main` rule,
> its push to `main` was accepted (`6ef15bb`, an empty commit, left in place).
> The rule now applies to admins too, so the author's own changes reach `main`
> by pull request as well. **Still open:** acting as the author, an agent could
> open a pull request and merge it. A required approval cannot close this,
> since GitHub does not let an author approve their own pull request. Closing
> it needs a separate, lower-permission identity for builder agents. Until
> then, "merged by Gabriel" (D45) is agreed, not enforced.

### D47 — Code is read before it runs here

The cloud contains the agent; it does not contain the code. A cycle branch
arrives on this machine on checkout and **executes** on `cargo build` or
`cargo test`, including every dependency's build scripts and proc-macros (T9).

Before a cycle branch is built locally, its new crates and its `Cargo.lock`
diff are reviewed. S3 decides how that review is recorded. Until it does, no
cycle branch that adds a dependency is built on this machine.

---

## Rejected

- **Separate macOS user for builder agents.** Sound, but it keeps the private
  data on the same machine behind a boundary the author maintains. It stays a
  candidate for domain B (D32 step 1, the browser on real websites), which this
  ADR does not decide.
- **Claude Code's local sandbox as the main boundary.** Same objection. Kept
  as defence in depth for anything that does run locally (S8).
- **Local builder agents with an allowlist only (S2).** Permissions narrow what
  an agent can do; they do not take the private data out of its reach.

---

## Risks

### R9 — The Anthropic API is an exit the network settings do not close

Even with network access disabled, a cloud session talks to the Anthropic API,
which the documentation notes may let data leave the VM. Accepted: the VM holds
only a public repository. **Revisit if anything private ever enters a cloud
environment**, including network secrets or environment variables.

### R10 — D47 erodes into "it passed CI"

Reading a lockfile diff is tedious, and green checks feel like review. The same
failure mode as R8. D47 is written down so that skipping it is a visible
departure from a rule.

---

## Consequences

- S1 is split. For builder agents it is answered here. For the browser it
  stays open until rung 4.
- S2 shrinks: what builder agents may run matters less when nothing private
  can be reached. Network policy for the cloud environment remains to set
  (the default "Trusted" level allows a list of domains; confirm crates.io is
  on it and decide whether anything else is needed).
- S3 becomes the local boundary, and the gate (S9) should now read: *before
  Cycle 0, D46 is set up and S3 is answered.*
- Cloud sessions share the plan's usage limits.

Source: [Use Claude Code in the cloud](https://code.claude.com/docs/en/claude-code-on-the-web).
