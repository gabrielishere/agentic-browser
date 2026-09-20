# Threat model

**Status:** Living document. Threats are added as the surface grows.
**Last touched:** 2026-09-20

> Tracks the **attack surface**. Decisions about what to do are recorded in the
> [ADR series](../adr/) as normal `D`-numbers — one decision sequence, not two.
> This document numbers *threats* (`T1…Tn`) because a threat is not a decision.
>
> **Personal use is the higher-risk case, not the lower one.** No security team,
> no sandbox by default, running on the machine that holds everything else.
> There is no blast radius because the blast radius is the laptop.

---

## How to read this

Each threat carries:

- **Live from** — the phase at which it becomes real. A threat that is not yet
  live still gets an entry, so it is not discovered late.
- **Severity** — consequence if exploited, not likelihood.
- **Status** — `open`, `mitigated`, `accepted`, or `deferred`.
- **Addressed by** — the `D`-number that does something about it, if any.

A threat with no mitigation is recorded as `open`. **Silence is not mitigation.**

---

## Phase gates

Preconditions, not aspirations. The point of this section is to stop "harden it
later", which on a solo part-time project means never.

| Gate | Condition |
|---|---|
| **Before pointing the engine at any real website** | T1 and T2 mitigated. D32 step 1 (separate user account) in place. |
| **Before the archive stores any authenticated page** | T5 and T6 mitigated. |
| **Before any `act` tool exists** (click, type, submit) | T4 has a written answer. Not "be careful" — a mechanism. |
| **Before the repository is public** | T9 reviewed; no secrets in history. |
| **Before the first snapshot is saved** | T5 as amended has an answer — O8 resolved, denylist decided either way. |
| **Before Phase 2 begins** (a JavaScript engine, D19) | T11 mitigated. Every snapshot already stored is in scope, not just new ones. |

---

## Threats

### T1 — Parser exploitation by hostile input

- **Live from:** first real page fetched
- **Severity:** high — process compromise or denial of service on the dev machine
- **Status:** open
- **Addressed by:** D30 (no I/O in core limits reach), D32 (containment)

Parsers written by someone learning the language, fed deliberately malformed
input. Rust removes buffer overflows and use-after-free; it does **not** remove
panics, unbounded recursion, infinite loops, or memory exhaustion. A page
crafted to hang or OOM the engine is trivial to construct and requires no
sophistication.

Note that D30 limits the *reach* of an exploited parser but does not prevent
exploitation. Containment, not prevention.

### T2 — SSRF and local resource access

- **Live from:** the first `open(url)` call
- **Severity:** high — local file disclosure, internal network access
- **Status:** open
- **Addressed by:** nothing yet

An agent told "go to this URL" can be pointed at `file:///Users/...`,
`localhost`, `127.0.0.1`, link-local addresses, or internal network ranges. The
classic vulnerability, exploitable on day one with only a single tool, and
*more* dangerous here than on a server because the interesting files are on the
same machine.

Needs an explicit scheme allowlist and address-range denylist, applied before
the request is made and again after any redirect.

### T3 — Prompt injection into agent context

- **Live from:** the first `read()` returning page content
- **Severity:** medium while read-only; high once acting is possible
- **Status:** open
- **Addressed by:** nothing yet

A page can contain text that reads as instructions. The agent reads pages.
This is unsolved industry-wide, and no mitigation here should claim otherwise.

While the surface is read-only the consequence is bounded — an injection can
mislead the agent but cannot make it act. That bound disappears with T4.

### T4 — Injection becomes action

- **Live from:** the first `act` tool
- **Severity:** critical
- **Status:** deferred — gated, see Phase gates
- **Addressed by:** nothing yet

Once the agent can click and type, attacker-controlled text on a page can cause
attacker-chosen actions. If session sharing is ever enabled (§4 of the MCP
design, Q4), those actions are taken **as the user**, with their logins.

This is the threat that justifies read-only being the Phase 1 default, and the
one that must have a real mechanism — not vigilance — before acting ships.

### T5 — The archive as a concentrated target

- **Live from:** the first capture
- **Severity:** **critical** (raised from high by ADR 0006 D33), rising monotonically with use
- **Status:** open
- **Addressed by:** nothing yet

> **Amended 2026-09-20 by ADR 0006 D33.** This entry originally assumed captured
> content resembled an anonymous crawl. Capturing the *rendered DOM as viewed*
> makes that false: authenticated, logged-in content enters the archive.
> Personal email, banking, medical records and internal tools are all
> capturable, in a way no anonymous crawler could reach. Severity raised
> accordingly, and O8 (a mandatory domain denylist) becomes pressing rather
> than optional.

The project deliberately builds a single store of everything the author has
read. That is the product's value and simultaneously its worst liability: one
file, on disk, containing personal reading history and potentially the contents
of authenticated pages.

Severity grows over time, which makes it easy to under-rate at the start. It is
least dangerous on the day it is built and most dangerous years later.

### T6 — Session credentials at rest

- **Live from:** the first stored cookie or login
- **Severity:** critical
- **Status:** open
- **Addressed by:** nothing yet

Cookies and session tokens are bearer credentials. Stored carelessly they are
worth more to an attacker than the archive itself, and they sit in the same
place.

Interacts with D14 (archive state and session state kept separate) — that
separation is the precondition for treating them differently, and should be
respected for this reason as much as for the architectural one.

### T7 — Headless browser dependency

- **Live from:** first capture (D18 — capture via headless browser)
- **Severity:** high
- **Status:** accepted, with the reason recorded
- **Addressed by:** D18 (states the trade), D32 (containment)

Capture runs a full headless browser over untrusted input by design. That is a
very large attack surface, not under this project's control, and updated on
someone else's schedule.

Accepted because it is convention across this entire problem domain (ADR 0002
R7) and because writing a JavaScript engine is out of scope. Recorded as
accepted rather than mitigated, since nothing here reduces it.

### T8 — Resource exhaustion

- **Live from:** the first agent-driven fetch
- **Severity:** medium — disk exhaustion, unusable machine
- **Status:** open
- **Addressed by:** nothing yet

An agent in a loop fetching and capturing will fill the disk. No malice
required; a retry bug is sufficient. Needs bounds on capture size, total store
size, and request rate.

### T9 — Supply chain

- **Live from:** the first dependency
- **Severity:** high
- **Status:** open
- **Addressed by:** nothing yet

Rust crates plus the headless browser. `cargo` pulls transitive dependencies
that run build scripts at compile time on the dev machine. Relevant to the
"before the repository is public" gate for the opposite reason too: nothing
secret should enter the history.

### T10 — Archive poisoning and replay

- **Live from:** the first `search()` over stored content
- **Severity:** medium
- **Status:** open
- **Addressed by:** nothing yet

Non-obvious, and specific to this design. Injected content captured today is
**stored**, and replayed into an agent's context on some later search — long
after the page is forgotten and the original context is gone.

Unlike live browsing, where injection is transient, the archive makes it
durable. A page read once can attack an agent repeatedly, months later. This is
a threat the prior art (ArchiveBox, SingleFile) does not have, because their
output is not fed to a model.

### T11 — Stored scripts execute on later replay

- **Live from:** Phase 2 (D19, a JavaScript engine). Content accumulates from the first capture.
- **Severity:** high
- **Status:** open
- **Addressed by:** nothing yet

A captured DOM may contain `<script>` elements. They are inert in Phase 1,
which has no JavaScript engine — and become live the moment Phase 2 arrives.

**A page archived today becomes executable code later.** The archive quietly
accumulates attacker-supplied scripts against the arrival of an engine to run
them, and the usual defence — "that site was fine when I visited it" — does not
apply, because the script is preserved regardless of what the site does
afterwards.

Interacts with T10. Archived content is already durable and replayed into an
agent's context long after the page is forgotten; T11 extends that from text to
code.

Candidate mitigations, none decided: strip scripts at capture; store but never
execute on replay; explicit opt-in per snapshot.

The trigger date is known — it is whenever Phase 2 begins — which makes this a
scheduled problem rather than a surprise. See Phase gates.

---

## Not yet assessed

<!-- Park here rather than losing it -->

- Tool-specific threats, pending the minimal toolset (MCP design §2)
- Multi-agent / concurrent session interactions
- What an attacker who obtains the archive file can do with it
- Update and patch strategy for the headless browser dependency
- Whether MCP transport itself (stdio vs HTTP) introduces exposure
