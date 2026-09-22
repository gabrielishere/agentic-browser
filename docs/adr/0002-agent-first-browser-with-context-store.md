# ADR 0002 — An agent-first browser with an integrated context store

- **Status:** Accepted
- **Date:** 2026-09-20
- **Deciders:** Gabriel
- **Builds on:** [ADR 0001](0001-browser-as-rust-learning-vehicle.md)
- **Resolves:** O1 (purpose), O2 (JavaScript), O3 (GPUI), O4 (reimplement vs assemble). O2–O4 were raised in discussion and not recorded in ADR 0001; the names here are their only record.
- **Amended by:** [ADR 0003](0003-learning-is-primary-product-framing-is-instrumental.md) — the product language below ("product", "ships", "Phase 1/2") is instrumental, not a commitment. Scope decisions here stay authoritative; where they conflict with the learning goal, ADR 0001 takes priority. See 0003 D23.
- **Amended by:** [ADR 0007](0007-build-order-and-optional-features.md) D42 — D6 below: store and retrieve become an optional Cargo feature rather than part of every build.
- **Amended by:** [ADR 0008](0008-agents-build-author-specifies-and-reviews.md) D43 — D15's *write* column means built in this repository, by the agent method, not by hand.
- **Next:** MCP tool surface design — see O5.

---

## Context

ADR 0001 affirmed a browser as the vehicle for learning Rust and systems
computing, but deliberately left one question open: **what is the browser
for?** That question governs scope, because a browser is unbounded by default
and the only real lever is deciding what to refuse.

This ADR answers it.

### How agents consume URLs today

Two paths exist, and both are compromises in opposite directions.

**The thin path** (e.g. WebFetch) is essentially an API call: HTTP GET, convert
the returned HTML to markdown, run it through a model to extract what's
relevant. No browser is involved. Consequently there is no JavaScript
execution — a client-rendered SPA returns an empty shell — no session or
cookies, no interaction, no layout, and many sites reject the request outright.

**The thick path** (Playwright, usually via an MCP server) drives a real
headless Chromium. JavaScript runs, sessions persist, clicking and typing
work. Modern implementations return an accessibility tree rather than
screenshots, which helps considerably. But it is slow, brittle, and
heavyweight — and conceptually it is a puppet-string layer on top of a browser
built for human eyes. The engine parsed the HTML, built a DOM, computed
layout, and rendered pixels for a human; Playwright then reaches back in to
recover the structure the engine already had and discarded.

### The gap

**No browser exists whose native output is agent-shaped.** One option is too
thin, the other too thick. That gap is the opportunity, and it is the one axis
of this project where the author is not a beginner.

---

## Decisions — purpose and shape

### D5 — Purpose: an agent-first browser with an integrated context store

*Resolves O1.* Not a general-purpose browser. Scope is set by this opinion,
and anything that does not serve it is refused.

### D6 — The agent surface and the context store are one substrate

They are not two projects. Both require the same pipeline: fetch → parse →
store → retrieve. The agent needs a good representation of a page in order to
act on it; the store needs a good representation in order to keep it. One piece
of engineering, two consumers, each making the other more valuable.

The compounding is the point: an agent that can search *this user's* library is
more useful than one that can search the web, because the library is already
filtered by the user's own judgement.

### D7 — MCP is the integration surface

The browser exposes itself to agents natively over MCP — navigate, read, act,
query history — rather than being puppeteered through a human-facing UI.

### D8 — A context store, not bookmarks

A bookmark stores a *pointer*. Pointers rot, carry no content so cannot be
searched, and record nothing about why they were saved. This stores the thing
itself:

- Content as captured, immune to link rot, later edits, and paywalls that
  descend after the fact
- Full-text search across everything ever read — the feature that actually
  changes behaviour
- Capture context: what the user was doing, what led there, what else was open
- Relationships: what cites what; what was read in one session
- Annotations anchored to text, surviving changes to the live page

**Prior art, named honestly.** ArchiveBox does much of this self-hosted;
SingleFile captures pages as one file; Hypothesis does annotation; Readwise
Reader does read-later well. Every one of them sits *outside* the browser, as
an extension or external tool, and therefore re-derives structure the engine
already computed and threw away. Being inside the engine is the differentiator.

### D9 — Runs as a real desktop browser; human feature set deliberately thin

Minimum viable human surface: URL bar, back/forward, viewport, find-in-page.

Rationale: a renderer cannot be developed blind — eyes on layout and paint are
required to know they work. "Thin but real" is a forcing function: if a human
can browse with it, the engine genuinely works.

### D10 — The human UI is a client of the same internal API the MCP server exposes

One engine, two front ends, one interface between them. This prevents divergent
code paths, makes the MCP surface load-bearing for daily use rather than an
afterthought, allows any agent action to be observed by a human, and makes the
browser inspectable by construction.

---

## Decisions — architecture

### D11 — A conventional render pipeline

```
  URL
   │
   ▼
 NETWORK ──────────► raw bytes
   │
   ▼
 PARSE HTML ───────► DOM tree
   │
   ▼
 PARSE CSS ────────► stylesheets
   │
   ▼
 STYLE / CASCADE ──► computed style per element
   │
   ▼
 LAYOUT ───────────► geometry: position + size of every box
   │
   ▼
 PAINT ────────────► drawing commands
   │
   ▼
 COMPOSITE ────────► layers → GPU → pixels
```

This is what WebKit, Blink, Gecko and Servo all do, modulo vocabulary.

**Superseded alternative, recorded so the reasoning survives.** An earlier
proposal in discussion was to normalize pages into a bespoke format at capture
time and render only that format, on the theory that it would collapse scope by
never exposing the renderer to the adversarial real web. This was **considered
and rejected** for two reasons. First, the live pipeline *is* the thing being
learned; bypassing it yields a document viewer, not an education. Second, it
buys less than claimed — with `html5ever` doing the parsing either way, the
renderer receives a real DOM regardless, so normalization relieves CSS
complexity but not HTML complexity, and D12 relieves CSS complexity far more
simply.

### D12 — Scope controlled by CSS subset, not by intermediate format

Support block and inline layout, the box model, and a few dozen properties.
Defer tables, grid, floats, transforms, `position: sticky`. Implement more over
time.

This is what early-stage engines genuinely do. The pipeline stays conventional
and recognisable; the scope stays tractable.

### D13 — Storage: files plus SQLite

- **Content blobs on disk**, content-addressed by hash (`blobs/ab/cd/abcd…`).
  Large, immutable, write-once. Deduplication comes free: the same page
  captured twice is stored once.
- **SQLite** for metadata, relationships, annotations and the index. **FTS5**
  provides full-text search with no additional dependency — precisely the
  killer feature, at no cost.

Not a server database. Single-user and local-first means no concurrency or
network problem to solve, and Postgres would be pure overhead. Backup is
copying one file.

`sqlite-vec` can add vectors to the same file later if semantic search is
wanted. Deferred — full-text search goes further than people expect.

### D14 — Two kinds of state, kept separate

- **Archive state** — captured pages. Durable, append-mostly, never invalidated.
- **Session state** — cookies, logins, current page, agent session context.
  Mutable, short-lived, different durability expectations.

Both may live in SQLite, but conflating them in the design causes pain later.

### D15 — Build principle: import the spec-grinding, write the ideas

*Resolves O4.*

| Import | Write |
|---|---|
| `html5ever` — HTML parsing | Style resolution and cascade |
| `swash` — text shaping | Layout |
| `wgpu` — GPU paint | Paint |
| `winit` — windowing | Storage and indexing |
| | The MCP surface |

Reimplementing HTML error recovery teaches nothing — it is spec-grinding, and
the result would be worse. Font table formats and GPU API surfaces are
implementation brutality rather than concepts. Layout, by contrast, is where
the ideas live and is the single best thing in the stack to understand deeply.

### D16 — Zed is a reference, not a dependency

*Resolves O3.*

Zed's role is as a reference implementation to **read**: `crates/sum_tree/` and
`crates/rope/` for how fast editors work, `crates/gpui/` for what a layout and
paint stack looks like in Rust. Nothing obliges this project to follow Zed's
architecture, and it should dictate its own.

**GPUI is declined as a dependency.** Licensing would permit it (Apache-2.0),
but it is tightly coupled to Zed's needs, and depending on it would mean
inheriting someone else's abstractions in precisely the area where the learning
lives. `winit` plus `wgpu` is more independent, more standard, and more
educational.

---

## Decisions — JavaScript

### D17 — "Supporting JavaScript" is three problems, not one

1. **A JS engine** — parses and executes the language. *Not written here.*
   Nobody writes one, including Servo, which embeds SpiderMonkey. Embedding an
   existing engine is on the order of weeks.
2. **DOM bindings** — the engine must reach into this project's tree so
   `document.querySelector()` finds real nodes. Substantial work.
3. **Web APIs** — `fetch`, `setTimeout`, `localStorage`, `addEventListener`,
   `history`, `MutationObserver`, and hundreds more, used liberally and
   unpredictably. **This is the part that never ends.**

Underneath all three: an event loop, a microtask queue, and invalidation, since
JS mutates the DOM and forces re-style, re-layout and re-paint.

Honest assessment: running scripts is achievable. Making modern sites *work* is
not — not for one person part-time. Servo has had thirteen years and funded
teams and is still not fully compatible.

### D18 — Phase 1: no JavaScript in the engine

*Resolves O2.*

Capture pages with a real headless browser and store the **post-JS DOM**. The
JavaScript has already run by the time content reaches the archive, so the
engine renders static HTML.

This is domain convention, not a workaround: ArchiveBox uses headless Chrome,
SingleFile runs inside a real browser, and Google's crawler renders with
headless Chrome before indexing. Everyone extracting from the modern web does
this.

In Phase 1 the archive, the context store and the MCP surface are all fully
functional. No learning is forfeited, because capture was never where the
education lived — parse, style, layout and paint are untouched.

### D19 — Phase 2: embed a JavaScript engine for live interactivity

Assessed options:

| Engine | Binding | Assessment |
|---|---|---|
| **QuickJS** | `rquickjs` | Small, embeddable, comprehensible, solid ES2020. **Current front-runner** for a learning project |
| V8 | `rusty_v8` | The "real" answer, used by Deno. Fast, mature, very large dependency, complex bindings |
| JavaScriptCore | — | Already present on macOS as a system framework. Bindings less polished |
| SpiderMonkey | `mozjs` | What Servo uses. Painful to build |
| Boa | — | Pure Rust, lovely idea, not yet capable of real sites |

Choice deferred until Phase 2 is actually triggered — see O6.

---

## Risks

### R5 — Thin-UI discipline will erode

Every missing affordance irritates in daily use. Tabs, history UI, a bookmarks
bar, settings, downloads — each individually reasonable, collectively a second
project that teaches nothing.

*Proposed rule:* **no human feature ships unless it is needed to debug the
engine, or is a thin client over an existing MCP capability.**

### R6 — Phase 1 cannot browse most of the live web

A browser without JavaScript cannot render most modern sites. In Phase 1 the
human side realistically covers the archive plus static and server-rendered
sites — which is more than commonly assumed (documentation, Wikipedia, many
news sites, most blogs), but is a real limitation.

This is deferred, not dodged. Live interactive browsing genuinely requires
Phase 2.

### R7 — The headless-browser capture dependency is large

It is a heavyweight dependency and it feels like cheating.

*Mitigation:* it is the convention across this entire problem domain, and
capture was never where the learning was.

### Inherited from ADR 0001

R1 (no natural finish line), R2 (unbounded scope), R3 (sequencing — north star,
not first step), R4 (hardware ceiling on outlier workspaces).

---

## Open items

### O5 — The MCP tool surface is unspecified

**This is the actual novel contribution of the project and it is currently
blank.** What tools are exposed, what shape they take, what a model receives in
response, how sessions and state are addressed. Likely warrants its own ADR.

### O6 — Phase 2 trigger

What conditions justify taking on JavaScript, given D17's honest assessment of
the cost.

### O7 — Project name

---

## Consequences

- Purpose decides what to refuse, which is the only mechanism that makes a
  browser tractable.
- The novelty sits in storage, retrieval and the MCP surface. The rendering is
  deliberately conventional — and conventional is exactly where the learning
  is.
- Phase 1 is a complete and useful product without solving the hardest problem
  in the space.
- Thin-UI discipline becomes a standing commitment, not a one-time decision.
- Zed's status is fixed as reading material, freeing the project from any
  obligation to its architecture.
