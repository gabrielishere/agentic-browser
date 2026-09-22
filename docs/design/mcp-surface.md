# MCP surface — design

**Status:** **Parked** until a working engine exists ([ADR 0007](../adr/0007-build-order-and-optional-features.md) D40). The core of §2 is settled; everything else is open.
**Last touched:** 2026-09-22

> This is a **living document**, not an ADR. It describes what the contract
> *is*, and is edited freely as the design moves. When a choice hardens into a
> decision with a rationale worth preserving, it graduates to an ADR and this
> document keeps only the result.
>
> Rule of thumb: if you'd phrase it *"we chose"*, it belongs in an ADR. If
> you'd phrase it *"it is"*, it belongs here.

**Context:** [ADR 0002](../adr/0002-agent-first-browser-with-context-store.md)
D7 (MCP is the integration surface), D10 (human UI is a client of this same
API). This document fills open item O5.

---

## 0. The design lens

The question that should decide every choice below:

> **What does owning the engine let us do that puppeteering a browser cannot?**

Anything Playwright already does well is table stakes. It is not worth building
for its own sake, and it is not where this project earns its existence.

Four candidate advantages, identified but not yet designed:

1. **Diff-based reads** — the engine knows exactly what invalidated, so it can
   return only what changed since the agent last looked. A puppeteer must
   re-snapshot everything.
2. **Stable node identity** — nodes can carry identities that survive
   re-render, so an agent can hold a handle across turns.
3. **Layout-aware representation** — geometry has just been computed, so
   visibility, reading order, above-the-fold and occlusion are known facts
   rather than inferences from screenshots.
4. **Browse and recall in one surface** — live web and personal archive behind
   the same interface. Nothing else can do this.

---

## 1. The read representation

> **The central artifact. Everything else hangs off it.**
>
> Constraint to hold onto: whatever `read` returns must double as the
> addressing scheme for `act`. If the agent reads "button: Submit", it must be
> able to say "click *that*". Representation and interaction handle are one
> design decision, not two.

<!-- TODO
  - What shape? (a11y-tree-like / custom semantic tree / something else)
  - What's in a node: role, name, text, geometry, state, children?
  - What's deliberately omitted, and why
  - Token cost per page — this is a real budget, measure it
  - How does it degrade on a huge page?
-->

### 1.1 Prior art to measure against

<!-- TODO
  - Playwright MCP's snapshot + `ref` scheme: what works, what doesn't
  - Accessibility tree: what it knows, what it's missing (geometry)
  - Raw HTML / markdown: why both are wrong
-->

### 1.2 Node addressing and identity

<!-- TODO
  - Playwright refs are per-snapshot and renumber on re-read. Can we do better?
  - What makes an identity stable? Survives re-render? Survives navigation?
  - What happens when an addressed node disappears?
-->

### 1.3 Diffs

<!-- TODO
  - What's the unit of change?
  - How does the agent ask for "what changed"?
  - What if the agent's last-known state is too old?
-->

---

## 2. Tool inventory

> Signatures and schemas. Kept current; this is the contract other things code
> against.

**Settled so far — three tools, read-only:**

```
open(url)     -> handle           fetch and parse; nothing is stored
read(handle)  -> representation   Q1 — return type undecided
search(query) -> hits             Q5 — hits are handles, or content?
```

- **Stateless.** Every call names its target. No current page. Costs tokens per
  call; buys composability, two pages held at once, and the same addressing for
  live and archived content.
- **`back` and `forward` are unnecessary**, and fall out of statelessness
  rather than being cut — the agent still holds earlier handles, so going back
  is reading something it already has.
- **No `save`.** Saving is a human action, not a tool (ADR 0006 D38). With it
  removed, the remaining surface is read-only by nature rather than by
  restriction.
- **No `act`.** Follows from the above and from Phase 1 having no JavaScript
  engine (D18). T4 gates its introduction.

Threat surface per tool:

| Tool | Exposes |
|---|---|
| `open` | T2 (SSRF), T1 (parser), T8 (exhaustion), T7 (headless dependency) |
| `read` | T3 (injection enters agent context) |
| `search` | T10 (poisoning replayed), T3 |

`open` carries most of the security work: scheme allowlist, address denylist
re-checked after redirects, size cap, timeout. T2's gate makes it blocking.

### 2.1 Navigate

<!-- TODO: signature and schema for open(); reload? -->
<!-- back/forward deliberately absent — see above -->


### 2.2 Read

<!-- TODO: read whole page, read diff, read region? (no "current page" — stateless, see above) -->

### 2.3 Act

<!-- TODO: click, type, submit, scroll, select -->

### 2.4 Archive and recall

> The differentiator. No other browser MCP can offer this.

<!-- TODO
  - search(query) — full text over everything captured
  - related(url) — what was read around this
  - temporal recall — "what was I reading last Tuesday"
  (No capture tool: saving is a human action, ADR 0006 D38.)
-->

### 2.5 Session

<!-- TODO: identity, cookies, login state -->

---

## 3. Granularity

> Open: many small tools, or few chunky ones?

<!-- TODO
  Round trips are expensive — each is a full model inference — which argues
  chunky. Too chunky and the surface becomes a DSL the model must learn, which
  argues chatty. Current lean: chunky but not clever.

  Decide against worked transcripts (§5), not in the abstract.
-->

---

## 4. Session model

> Open: does the agent share the human's browser state, or get its own?

<!-- TODO
  Shared  — agent sees your logins and cookies. Far more useful, far more
            dangerous.
  Isolated — safe, but cannot act as you.

  Current lean: isolated by default, explicit opt-in to share.
  Has product consequences, not just security ones.
-->

---

## 5. Worked transcripts

> **The primary evaluation technique for this document.**
>
> Write out a realistic agent session, turn by turn — every call, every
> response. Design flaws invisible in a schema become obvious in a transcript.
> Cheap to write, catches the expensive mistakes early.
>
> A design is not ready until its transcripts read as unpainful.

### 5.1 Scenario — find something on a live site

<!-- TODO -->

### 5.2 Scenario — multi-step interaction (search, filter, extract)

<!-- TODO -->

### 5.3 Scenario — recall from the archive

<!-- TODO -->

### 5.4 Scenario — something goes wrong (element gone, page changed, nav failed)

<!-- TODO: error paths matter as much as happy paths -->

---

## 6. Security

> Flagged, not solved. Recorded here so it isn't discovered late.

<!-- TODO
  Prompt injection stops being theoretical the moment an agent with live
  sessions reads attacker-controlled text. A page can contain content that
  reads as instructions; the agent is reading pages.

  Phase 1 mitigation candidate: read-only, no act tools.
  Longer term: unresolved. Confirmation gates? Capability scoping? Origin
  trust?
-->

---

## 7. Open questions

- **Q1** — What shape is the read representation? *(§1, blocks everything else)*
- **Q2** — Can node identity be made stable across re-render, and at what cost?
- **Q3** — Chunky or chatty tools? *(decide via §5)*
- **Q4** — Shared or isolated session state? *(cookies and logins — distinct from the stateless/stateful question, which is settled)*
- **Q5** — Does `search` return handles or content? Handles unify live and archived under one `read`; content splits the surface into two. *(sharpened from "is the archive queried through the same tools")*
- **Q6** — What does the engine do that a puppeteer genuinely cannot? *(keep re-asking; it's the reason the project exists)*
- **Q7** — What shape do errors take? `open` fails constantly — 404, timeout, TLS, blocked by the site, refused by the scheme allowlist (T2). The agent must distinguish *gone* from *forbidden* from *retry*, because the correct next action differs. Not a tool; a return shape, and currently blank.
- **Q8** — What does `read` do with an oversized page? Everything and blow the context window, or truncate and silently lose content? May require a fourth tool, a parameter, or pagination. *(depends on Q1)*
- **Q9** — Is there a gap for URL lookup? *"Do I have an archived copy of this exact URL?"* is a lookup, not full-text search — and *"have I read this before?"* is a natural and valuable question. Might be `search` with a URL-shaped query, might want its own tool.

> **Q1 keeps surfacing from every direction** — from the tool signatures, from
> oversized pages, from addressing. That is usually the sign of the genuinely
> load-bearing question.
>
> Proposed method when it is taken up: pick one real page, decide what the ideal
> `read` response looks like for that page specifically, and generalise from
> there. Designing the shape top-down tends to produce something tidy that fails
> on contact. Choose an awkward page — a well-structured article will not stress
> anything.

---

## 8. Not yet considered

<!-- Park things here rather than losing them -->

- Streaming vs request/response for long page loads
- Multiple concurrent agent sessions
- How the human UI consumes this same API in practice (ADR 0002 D10)
- Rate limiting / resource bounds on agent actions
