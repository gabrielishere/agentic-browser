# TODO

**Last updated:** 2026-09-22

> Sequenced work, not an exhaustive list. Open *questions* live in the documents
> that own them — this file points at them rather than restating them, so the
> two cannot drift apart.

---

## Resuming cold

Read in this order:

1. [`README.md`](../README.md) — what this is
2. [ADR 0002](adr/0002-agent-first-browser-with-context-store.md) — purpose and architecture
3. [ADR 0006](adr/0006-snapshots-are-rendered-dom.md) — what a snapshot is
4. [`design/mcp-surface.md`](design/mcp-surface.md) §2 — the tool surface as it stands

Everything decided is in `docs/adr/`, D1–D38, contiguous. Everything undecided
is either a `Q` in the MCP design document or an `O` in an ADR.

---

## Now — finish the MCP surface

### 1. §1 — the read representation  ← **start here**

**Q1.** The load-bearing question. Blocks Q2, Q8, and half of Q5. It has
surfaced from every direction we approached the design from, which is the usual
sign.

Method agreed: **pick one real page**, decide what the ideal `read` response
looks like for that page specifically, then generalise. Designing the shape
top-down produces something tidy that fails on contact. Choose an awkward page
— a search results page, a GitHub issue, an app-like page. A well-structured
article will not stress anything.

The constraint to hold throughout: whatever `read` returns must double as the
addressing scheme for any future `act`. Representation and interaction handle
are one decision.

### 2. §1.1 — prior art

Measure against Playwright MCP's snapshot-plus-`ref` scheme, the accessibility
tree, raw HTML, markdown. Each fails on a different axis; the two nobody has
are **geometry** and **incremental reads**, which are exactly what owning the
engine provides.

### 3. §1.2 — node addressing and identity  *(Q2)*

Playwright's refs renumber on every snapshot. Can identity survive re-render?
Navigation? What happens when an addressed node disappears?

### 4. §1.3 — diffs

Unit of change, how the agent asks, what happens when its last-known state is
too old.

### 5. §2 — per-tool signatures

`open` (§2.1) and `read` (§2.2) only. **§2.3 (act) and §2.5 (session) stay
dormant** while the surface is read-only. §2.4 (archive and recall) depends on
Q5 and Q9.

Also settle **Q7** here — the error shape. `open` fails constantly and the agent
must distinguish *gone* from *forbidden* from *retry*.

### 6. §5 — worked transcripts

**This is the evaluation method for everything above, not a later section.**
Write a realistic session turn by turn. Flaws invisible in a schema become
obvious in a transcript. §5.4 (something goes wrong) matters as much as the
happy paths, and validates Q7.

Q3 (granularity) is decided here, against real transcripts, not in the
abstract.

### 7. §6 — convert to a pointer

Security content now lives in [`security/threat-model.md`](security/threat-model.md)
as T1–T11. §6 should reference it, not duplicate it.

---

## Next — the architecture map

Not yet defined. Decide what it means before starting: a module and crate
layout, a data-flow diagram, the process and I/O boundary from D30, or all
three.

Constraints it must satisfy, already decided:

- **D30** — parse, style, layout and paint perform no I/O
- **D27** — core is a library, application is a thin shell
- **D10** — the human UI is a client of the same internal API as the MCP server
- **D31** — process separation must remain a refactor, not a rewrite
- **D11** — conventional pipeline: network → parse → style → layout → paint → composite
- **D13 / D37** — content-addressed blobs plus SQLite; custom container, WARC as export

---

## Blocked or deferred

| Item | Blocked on |
|---|---|
| **O8** — is a domain denylist mandatory? | Gated: must be answered *before the first snapshot is saved*. T5 is now critical |
| **O9** — retention policy | Nothing bounds archive growth |
| **O10** — is pre-JS HTML always stored? | Confirm against real storage cost |
| **Q4** — shared or isolated session state | Only matters once cookies or logins exist |
| **T11** — stored scripts | Gated: must be mitigated *before Phase 2 begins* |
| Anything `act`-shaped | T4 needs a mechanism, not vigilance |

---

## Housekeeping

- [ ] GitHub → Settings → Emails → **Block command line pushes that expose my email**
- [ ] Delete `/tmp/agentic-browser-git-backup-*` (pre-rewrite backup, holds the old hostname identity)
- [ ] Decide whether `docs/design/mcp-surface.md` §4 survives, given statelessness is settled
