# agentic-browser

Research into browser engine design for agent consumption: a native MCP
interface over the page representation, and a searchable local-first archive of
visited pages.

**There is no code.** This repository contains design decisions, a design
document for the tool surface, and a threat model.

---

## The problem

Two mechanisms exist for giving an agent a web page.

The first fetches HTML over HTTP and converts it to text. No JavaScript
executes, so client-rendered pages return an empty shell. There is no session,
no interaction, and no layout.

The second drives a headless Chromium through an automation framework such as
Playwright. JavaScript executes and sessions persist, at the cost of latency,
brittleness, and a large dependency. Structurally it is a control layer above a
browser built for human output: the engine parses HTML, builds a DOM, computes
layout and rasterises for a display, and the automation framework then queries
back into it to recover structure the engine had already computed.

Neither mechanism is a browser whose native output is intended for a program.

## Goal

An engine that produces an agent-addressable representation directly, exposed
over MCP, together with an archive of what has been read.

The archive stores captured content rather than URLs, indexed for full-text
search, with the circumstances of capture and the relationships between pages.
The two halves share a pipeline — fetch, parse, store, retrieve — so they are
treated as one piece of engineering with two consumers rather than as separate
components.

## Decisions taken so far

Recorded in [`docs/adr/`](docs/adr/), with the reasoning and the rejected
alternatives.

- A conventional render pipeline: network, parse HTML, parse CSS, style,
  layout, paint, composite. Scope is bounded by supporting a subset of CSS
  rather than by introducing an intermediate format.
- Storage as content-addressed blobs on disk with SQLite for metadata and
  indexing; FTS5 for search.
- MCP as the integration surface, with the human-facing UI implemented as a
  client of the same internal API.
- A deliberately minimal human feature set: URL bar, back and forward,
  viewport, find-in-page.
- No JavaScript engine in the first phase. Capture runs through a headless
  browser, so the archive holds post-JavaScript content.
- Parsing, style, layout and paint perform no I/O, which keeps process
  isolation available as a later refactor rather than a rewrite.

## Documentation

| | |
|---|---|
| [`docs/adr/`](docs/adr/) | Decisions, with rationale. Immutable; superseded rather than edited. |
| [`docs/design/`](docs/design/) | Living specifications. Edited freely as the design moves. |
| [`docs/security/`](docs/security/) | Threat model. Tracks the attack surface; decisions about it live in the ADRs. |

Start with [ADR 0001](docs/adr/0001-browser-as-rust-learning-vehicle.md) and
[ADR 0002](docs/adr/0002-agent-first-browser-with-context-store.md), then
[ADR 0003](docs/adr/0003-learning-is-primary-product-framing-is-instrumental.md),
which settles how the first two relate: the product shape sets *what* is in
scope, the learning goal sets *how* it is built.
[ADR 0004](docs/adr/0004-preserve-optionality-toward-product.md) covers the
few decisions that are cheap now and irreversible later.

The [MCP surface](docs/design/mcp-surface.md) is the novel contribution and is
currently unspecified.

## Context

This is a part-time personal project and an explicit learning exercise — its
author does not know Rust yet. Success is measured as understanding rather than
shipping. See ADR 0001 for that framing and the risks it carries.

It is public because open source was a deliberate choice rather than an
afterthought, not because it is ready for anyone to use.

## Licence

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this work shall be dual licensed as above, without any
additional terms or conditions.
