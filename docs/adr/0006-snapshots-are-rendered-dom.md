# ADR 0006 — Snapshots are rendered DOM, not raw HTTP transactions

- **Status:** Accepted
- **Date:** 2026-09-20
- **Deciders:** Gabriel
- **Builds on:** [ADR 0002](0002-agent-first-browser-with-context-store.md) D8 (a context store, not bookmarks), D13 (blobs plus SQLite), D18 (capture via headless browser); [ADR 0004](0004-preserve-optionality-toward-product.md) D28 (versioned, documented format)
- **Related:** [threat model](../security/threat-model.md) T5, T6, T8, T10, T11

---

## Context

The requirement, as stated: *a Wayback Machine style snapshot of the site at
current viewing.*

That phrasing contains an assumption worth examining, because the Wayback
Machine does not work the way it appears to.

### What the Wayback Machine actually stores

**It stores no DOMs.** Its crawler (Heritrix) fetches URLs anonymously — no
login, no session, no user. Each fetch becomes a **WARC** record holding the
request, the response headers, and the raw bytes the server sent. Subresources
— stylesheets, images, scripts — are crawled separately, each with its own
record and its own timestamp.

The archive therefore holds HTTP transactions, not pages.

### How replay reconstructs a page

On opening an archived URL, a CDX index locates the capture closest in time to
the one requested. The stored HTML is served, every URL within it is rewritten
on the fly to point back into the archive, and a client-side shim is injected
(pywb's `wombat.js` being the well-known open-source example) to intercept
runtime `fetch`, `XHR` and `document.write` calls and redirect those into the
archive as well.

**JavaScript executes at replay, in the viewer's browser, now.** The archive
never ran it; it stored the script file. This is why archived pages frequently
render partially — the script runs, issues requests, and those requests meet
archived responses that may be missing or years out of step.

### Temporal violation

Because "closest available capture" is resolved independently per resource, a
page archived in January may assemble with a stylesheet from the previous June
and an image from two years later. Web archiving research names this *temporal
violation*. **What the viewer sees was never a state the site was in.**

### Why none of this applies here

Every constraint that shaped that architecture is absent from this project.

| Wayback's constraint | This project |
|---|---|
| Archives the whole web at crawler scale | One user, pages personally viewed |
| Evidentiary value — exact server bytes matter | Personal recall |
| Cannot execute JavaScript across billions of pages | D18 already runs a headless browser at capture |
| Replays into a browser it does not control | Owns the renderer |

The rewriting shim, the runtime interception and the per-resource timestamp
resolution are the hardest parts of that system, and they exist **solely** to
make a foreign browser resolve archived content. That cost buys nothing here.

---

## Decisions

### D33 — Snapshots are rendered DOM, not raw HTTP transactions

A snapshot is the post-JavaScript DOM as it stood at the moment of capture,
together with its subresources.

**Wayback's replay model is explicitly rejected.** Adopting it would mean
building the rewriting and interception machinery for no benefit, discarding
the post-JavaScript DOM that D18 already produces, and importing temporal
violation as a defect.

The trade, stated plainly: the original byte stream is not the primary
artifact, so a snapshot cannot serve as evidence of what a server sent, and a
different rendering cannot be re-derived from it. D34 retains enough to blunt
this; full fidelity to the wire is given up deliberately.

### D34 — What a snapshot contains

| Component | Rationale |
|---|---|
| Post-JavaScript rendered DOM | the primary artifact — what was actually on screen |
| Subresources: CSS, images, fonts | a snapshot without them is text, not a page; content-addressed per D13, so deduplicated across snapshots and across a site |
| Original pre-JavaScript HTML | small beside images, therefore near-free; preserves the ability to re-derive |
| HTTP response headers | negligible in size, and wanted the first time something is inexplicable |
| Capture metadata | timestamp, URL, and why it was saved |

The final three rows are the Wayback instinct retained cheaply. Discarding them
would be a one-way door of exactly the kind ADR 0004 exists to keep open: the
pre-JavaScript HTML costs almost nothing next to a page's images, and dropping
it permanently forecloses deriving any other view of the page.

### D35 — Snapshots are atomic and timestamped

One moment, captured once, stored whole. Many snapshots of the same URL coexist
and are addressed by time.

This takes what the Wayback Machine gets right — timestamps as a first-class
axis — and refuses what it gets wrong. No snapshot here is assembled from
resources captured at different times.

### D36 — Rendering a snapshot performs no network access

Every reference resolves to a stored blob. If a resource was not captured, it
is absent; it is never fetched.

Three reasons, and each would be sufficient alone:

- **Privacy.** Otherwise, opening an old snapshot silently reports your reading
  to the origin server, years later.
- **Durability.** Otherwise, a snapshot degrades as hosts change or disappear,
  which defeats the purpose of having taken it.
- **Determinism.** A snapshot renders the same way every time.

### D37 — A custom container over content-addressed blobs; WARC as an export path

The internal format is this project's own, built on the content-addressed blob
store of D13, versioned and documented from the first write per D28.

**WARC is not the foundation.** It is a container designed for crawler output
and carries structure this project does not need. But an export path to WARC is
a goal, so that an archive can leave this software and be read by existing
tools. Data that only means something inside one program is a poor thing to
accumulate for years.

### D38 — Snapshot creation is a human action

Saving is performed by the user. It is **not** exposed as an MCP tool. The
agent may read and search; it cannot write to the archive.

This is what keeps the archive filtered by human judgement, which is the
property that makes searching it more useful than searching the web (D6). An
agent permitted to save would dilute a curated library into another crawl.

It also settles the shape of the agent-facing surface: with saving removed, the
remaining tools are read-only by nature rather than by restriction.

> *May relocate to a future MCP-surface ADR. Recorded here because it
> determines what enters the archive and why.*

---

## Threats

### T11 — Stored scripts execute on later replay *(new)*

A captured DOM may contain `<script>` elements. These are inert in Phase 1,
which has no JavaScript engine (D18) — and become live the moment D19's Phase 2
arrives. **A page archived today becomes executable code later**, at which
point the archive is a store of attacker-supplied scripts awaiting an engine.

Interacts with T10: archived content is already durable and replayed into an
agent's context long after the page is forgotten. T11 extends that from text to
code.

Candidate mitigations, none decided: strip scripts at capture; store them but
never execute on replay; require explicit opt-in per snapshot.

### T5 — amended

The existing entry assumed captured content resembled an anonymous crawl.
D33 makes that false: capturing the rendered DOM **as viewed** means
authenticated, logged-in content enters the archive. Personal email, banking,
medical records and internal tools are all capturable in a way an anonymous
crawler could never reach.

This materially raises T5's severity and makes O8 pressing.

---

## Open items

### O8 — Is a domain denylist mandatory rather than optional?

Given T5 as amended, should the system refuse to capture certain domains
outright? The alternative relies on the user remembering not to press save on
their banking page, which is not a control.

### O9 — Retention

Nothing bounds archive growth. D38 makes T8 unlikely, since only a human can
save — but "unlikely" is not "bounded", and an archive intended to last years
needs an answer eventually.

### O10 — Is the pre-JavaScript HTML always stored?

D34 says always. Worth confirming against real storage cost on image-heavy and
script-heavy pages before it hardens.

---

## Consequences

- The renderer consumes static HTML with local references. No rewriting shim,
  no runtime interception, no per-resource timestamp resolution — a large body
  of work avoided rather than deferred.
- Snapshot fidelity exceeds the Wayback Machine's by construction, because
  capture is atomic and JavaScript has already run.
- Fidelity to the wire is given up beyond what D34 retains. If evidentiary
  value is ever wanted, that is a new decision, not an adjustment.
- The format has a documented slot for raw responses from the first write,
  satisfying D28 and keeping D33's trade reversible in principle.
- T11 is now a scheduled problem with a known trigger date — the arrival of a
  JavaScript engine — rather than a surprise.
