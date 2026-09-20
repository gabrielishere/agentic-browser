# agentic-browser

An agent-first browser with an integrated context store, written in Rust.

**Status:** design. No code yet.

---

## What this is

Two paths exist today for giving an agent a web page, and both are compromises.
The thin path fetches HTML and converts it to text — no JavaScript, no session,
no interaction, no layout. The thick path puppeteers a headless Chromium
through Playwright — slow, brittle, heavyweight, and conceptually a
puppet-string layer over a browser built for human eyes, reaching back in to
recover structure the engine already computed and discarded.

**No browser exists whose native output is agent-shaped.** That gap is what
this is for.

Alongside it, a context store: not bookmarks, which save a pointer that rots,
but the page itself — content as captured, full-text searchable, with the
context of why it was saved and what it relates to. An agent that can search
your own library is more useful than one that can search the web, because the
library is already filtered by your judgement.

The two turn out to be one piece of engineering with two consumers.

## Shape

- Conventional render pipeline — network → parse → style → layout → paint →
  composite. Scope held by supporting a subset of CSS, not by inventing an
  intermediate format.
- Storage is content-addressed blobs on disk plus SQLite; FTS5 for search.
- MCP is the integration surface. The human UI is a client of the same internal
  API.
- Human feature set deliberately thin: URL bar, back/forward, viewport,
  find-in-page.
- Phase 1 runs no JavaScript. Capture uses a headless browser, so the archive
  holds post-JS content.

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
afterthought, not because it is ready for anyone to use. There is no code yet.

## Licence

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this work shall be dual licensed as above, without any
additional terms or conditions.
