# Security — design

**Status:** Thinking space. Scoped, not designed. Nothing here is decided.
**Last touched:** 2026-09-22

> This is a **living document**, not an ADR, and not the threat model.
>
> | Document | Answers |
> |---|---|
> | [`security/threat-model.md`](../security/threat-model.md) | What can go wrong — threats `T1…Tn` |
> | **this document** | What stands in the way — the defences, how they fit, how we know they work |
> | [`adr/`](../adr/) | What we chose, and why — decisions `D1…Dn` |
>
> Same rule of thumb as the MCP document: *"we chose"* graduates to an ADR; *"it
> is"* stays here. Open questions are numbered `S1…Sn` so they cannot be
> mistaken for the MCP document's `Q`s or the threat model's `T`s.

**Context:** [ADR 0005](../adr/0005-sandboxing-boundary-and-io-free-engine-core.md)
(I/O-free core, sandboxing by cost), [ADR 0007](../adr/0007-build-order-and-optional-features.md)
(build order — which surfaces exist when),
[ADR 0008](../adr/0008-agents-build-author-specifies-and-reviews.md) (agents build
the code, which makes the build pipeline an attack surface).

**Sources:** [`research/security-sources.md`](../research/security-sources.md) —
linked references for every section. Its §0 lists six places where the sources
contradict this draft; those are unresolved here until each is worked through.

---

## 0. The design lens

> **What stands between a hostile input and the author's machine — and how
> would we know if it stopped standing there?**

There are two adversaries, live at different times, and it is easy to design
for the famous one and miss the present one:

1. **The web, at runtime.** Hostile pages fed to the engine. Live from the first
   real page fetched (stage 1, rung 4).
2. **The build pipeline, at development time.** Agents with a shell on the
   author's account, the crates they pull in, and the text they read. **Live
   now** — from the first dispatched task (ADR 0008).

A third arrives later: **data at rest** — the archive (stage 3).

**Personal use is the higher-risk case** (threat model preamble): one machine,
no security team, and it holds everything else. The design target is therefore
*containment* — assume something will go wrong, and bound what it can reach.

---

## 1. Reference model — what an official browser has

The layers a production browser puts between a page and the machine, and where
this project stands against each. This table is the map; the sections below are
the territory.

| Layer | In a production browser | Here | Where designed |
|---|---|---|---|
| Process sandbox | Renderer runs in an OS sandbox (Seatbelt on macOS): no files, no network, no process spawning | **Absent.** Deferred by D31; kept possible by D30 | §3.7, ADR 0005 |
| Site isolation | One site per process; sites cannot read each other's memory | **Absent.** Not needed until logins or JS exist | §5, §6 |
| Memory safety | C++ with hardening, partial Rust adoption | **Largely present by construction** — safe Rust. Holes: `unsafe` in dependencies, the GPU driver | §3.4, §3.5 |
| Fuzzing at scale | Continuous fuzzing of every parser | **Absent.** Borrowed indirectly via mature crates (`html5ever`, `rustls`) | §8 |
| Security response | Teams, bug bounties, CVE process | **Absent.** One author, part-time | §7 |
| Automatic updates | Patches within days | **Absent.** Updated when rebuilt | §7 |
| Reputation lists | Safe Browsing blocks known-bad sites | **Absent** | §3.1 |
| OS handoff controls | Download quarantine, Gatekeeper, permission prompts, scheme confirmation | **Absent.** No downloads in D9 | §3.6 |
| TLS | Certificate validation, HSTS, CT | To be built on `rustls` | §3.2 |

---

## 2. The development environment  ← **live now; before Cycle 0**

> The present adversary. Agents run `Bash` with the author's permissions
> (`.claude/agents/*.md`). Nothing a web page can do to stage 1 compares with
> what a misdirected agent can do to the machine it runs on.
>
> Threats: T9 (supply chain), T12 (agent build pipeline).

### 2.1 Machine isolation

<!-- TODO
  - D32 step 1 — a separate macOS user. Designed for the browser; covers the
    agents equally. Does it host both? (S1)
  - What that account can reach: no keychain, no SSH keys, no personal files,
    no signed-in cloud accounts. Enumerate and check.
  - Claude Code's own sandboxing / permission modes — complement or
    replacement? (S8)
-->

### 2.2 Agent permissions

<!-- TODO
  - What the implementor and orchestrator may run: an allowlist, not open Bash (S2)
  - Network access during a run — needed for `cargo fetch`; needed for anything else?
  - Writes outside the repository: forbidden, and how that is enforced rather than asked
-->

### 2.3 Dependency intake

<!-- TODO
  - Every new crate is a decision: build scripts and proc-macros run arbitrary
    code at compile time (T9). Who approves, and where is it recorded? (S3)
  - `Cargo.lock` committed; changes to it reviewed like code
  - `cargo-audit` (known vulnerabilities), `cargo-deny` (licences, sources,
    bans) — in Cycle 0?
  - Minimum dependency set per crate; `engine` especially (D30, D42)
-->

### 2.4 What agents read

<!-- TODO
  - Prompt injection via READMEs, crate docs, web pages, issue text
  - Do agents fetch from the web at all during a run? If not, say so in the spec
  - Fixture pages written for tests are authored content — keep hostile
    fixtures clearly marked (§8)
-->

### 2.5 Repository hygiene

<!-- TODO
  - Public repo: secrets, identity, and what an agent may commit (T9 gate, passed 2026-09-22)
  - D45 — agents commit to a cycle branch; only the author merges to `main`
  - Nothing pushes except the author
-->

---

## 3. The engine at runtime — stage 1

> The minimal browser: no JavaScript, no MCP, no archive, no downloads.
> Remaining surface: URLs, the network, parsers, the GPU, and anything handed
> to the operating system.
>
> Threats: T1 (parser), T2 (local access), T8 (exhaustion), T13–T15.

### 3.1 The input boundary — which URLs are accepted

<!-- TODO
  - Scheme allowlist: `http`, `https` only. `file://`, `data:`, custom schemes refused
  - Re-checked after every redirect (T2)
  - Local and private address ranges — refused for the human UI too, or only for agents?
  - Reputation lists — out of scope, or a later feature?
-->

### 3.2 The network

<!-- TODO
  - TLS via `rustls`; certificate verification never disabled, not even in tests
    against real hosts (T15). Test fixtures served locally instead
  - Timeouts, maximum response size, maximum redirects
  - What is sent: user agent, cookies (none in stage 1), referrer
-->

### 3.3 Parsers and resource limits

<!-- TODO
  - Panics are containment failures, not crashes to shrug at: catch at the
    engine boundary? abort? (S7)
  - Limits: document size, DOM depth, stylesheet count, image dimensions,
    decoded image memory, layout box count (S7 — the numbers)
  - Recursion: layout and style are tree walks; hostile nesting depth
-->

### 3.4 `unsafe` code

<!-- TODO
  - `#![forbid(unsafe_code)]` in our crates? (S4)
  - Inventory of `unsafe` in dependencies (`cargo-geiger`); which are tolerated
  - Image and font decoders — historically the richest seam in real browsers
-->

### 3.5 The GPU

<!-- TODO
  - `wgpu` → Metal → driver. Pages cannot supply shaders (no WebGL/WebGPU), so
    the surface is geometry and textures: sizes, counts, allocation
  - Texture size caps from §3.3
-->

### 3.6 Handoff to the operating system

<!-- TODO
  - No downloads (D9). If that changes: quarantine attribute, no execution,
    no page-chosen paths (T13)
  - Links with unknown schemes: never passed to `open` (T14)
  - Clipboard, notifications, file pickers — none in D9; keep it that way
-->

### 3.7 Containment when a defence fails

<!-- TODO
  - D30 limits a compromised parser's reach — what does it reach in practice?
  - When D31 (process separation) and D32 step 2 (Seatbelt) become worth it —
    the trigger, stated observably
-->

---

## 4. The agent surface — stage 2  *(dormant)*

Designed in [`mcp-surface.md`](mcp-surface.md) §6 and the threat model's T2, T3,
T4 and T10. This document takes ownership of the *mechanisms* when stage 2
begins; until then, a pointer.

## 5. The archive — stage 3  *(dormant)*

<!-- TODO when stage 3 begins
  - T5, T6: encryption at rest? file permissions? what an attacker holding the
    file learns ("Not yet assessed" in the threat model)
  - O8: the domain denylist, gated before the first snapshot
  - T7: headless capture — never `--no-sandbox`
  - T11: scripts in stored snapshots
-->

## 6. JavaScript — phase 2  *(dormant)*

<!-- TODO when D19 triggers
  - The exploit class safe Rust removed comes back: QuickJS is C
  - Site isolation and process separation stop being optional
  - T11 gate: every stored snapshot in scope
-->

---

## 7. Updating and patching

<!-- TODO
  - Dependency updates: cadence, who runs them, how they are reviewed
  - Advisories: how the author hears about a vulnerable crate (cargo-audit in CI?)
  - The headless browser (stage 3) — patched on someone else's schedule (T7)
-->

---

## 8. Evaluation — how we know a defence works

> **The equivalent of the MCP document's worked transcripts.** A defence without
> a test is a belief. Every mechanism above should name what would fail if it
> were removed — the same discipline the builder method applies to evidence
> (`.claude/builder/templates/prompt-template.md`, "Evidence must discriminate").

<!-- TODO
  - Hostile fixture corpus: deep nesting, huge documents, malformed UTF-8,
    `file://` links, redirect chains to localhost, oversized images
  - One test per defence, asserting the attack does not succeed — the effect,
    not the mechanism
  - Fuzzing: `cargo-fuzz` on the HTML → DOM and CSS → style paths. From which
    rung? (S5)
  - Where security tests live, and whether the orchestrator's gate can see them (S10)
-->

---

## 9. Open questions

- **S1** — Does the separate macOS user (D32 step 1) host the agents as well as the browser? *(Recommended: yes — one boundary covers both adversaries.)*
- **S2** — What may agents execute? The shape of the allowlist, and whether network access is granted beyond `cargo fetch`.
- **S3** — How is a new dependency approved and recorded — spec `Constraints`, a `cargo-deny` config, or both?
- **S4** — Is `unsafe` forbidden in this project's own crates?
- **S5** — When does fuzzing start — rung 1, or once `html5ever` is in (rung 2)?
- **S6** — Does the human UI refuse local and private addresses, or only the agent surface?
- **S7** — Resource limits: the actual numbers, and whether a panic in the engine aborts the process or is caught at the boundary.
- **S8** — Claude Code's sandboxing and permission modes: complement to the separate user, or replacement for it?
- **S9** — Should a phase gate precede Cycle 0 — *"before any agent runs a build: S1–S3 answered"*?
- **S10** — Where do security tests live, and how does a spec require them?

> **S1–S3 and S9 are the load-bearing ones** — they concern the adversary that
> is live now. Everything in §3 has until rung 4 touches a real website.

---

## 10. Not yet considered

<!-- Park things here rather than losing them -->

- Privacy as distinct from security — what a stage 1 fetch reveals about the author
- Code signing and notarisation, if a build is ever shared
- The MCP transport (stdio vs HTTP) — also parked in the threat model
- Backups of the archive, and whether a backup is a second copy of T5
