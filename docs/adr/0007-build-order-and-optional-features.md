# ADR 0007 — Build order: a minimal browser first; MCP and the archive as features

- **Status:** Accepted
- **Date:** 2026-09-22
- **Deciders:** Gabriel
- **Builds on:** [ADR 0003](0003-learning-is-primary-product-framing-is-instrumental.md) D22 (understanding decides *how*), [ADR 0004](0004-preserve-optionality-toward-product.md) D27 (library core), [ADR 0005](0005-sandboxing-boundary-and-io-free-engine-core.md) D30–D32
- **Amends:** [ADR 0002](0002-agent-first-browser-with-context-store.md) D6 (one substrate) — see D42
- **Parks:** [MCP surface design](../design/mcp-surface.md), except node identity — see D40

---

## Context

The TODO as of 2026-09-20 opened with *finish the MCP surface*: design the read
representation (Q1) on paper, then the tool signatures, then transcripts. No
engine code existed, and none was sequenced before that design work.

That is the drift ADR 0003 warned about. Its own table asks *"Build renderer or
MCP surface first?"* and answers **renderer** under the learning frame — which
D20 makes primary. MCP-first is the product frame's answer, and it had quietly
become the plan.

There is also a practical cost. The agreed method for Q1 is *pick one real page
and decide what the ideal `read` looks like for it*. That is far easier with a
real DOM and layout tree in hand than against an imagined one. Designing the
surface before the engine exists means designing it twice.

Separately, the sandboxing rules in ADR 0005 were written as a set. They are not
equally urgent, and treating them as one block makes the cheap, structural rule
look like ceremony.

---

## Decisions

### D39 — Build order: minimal browser, then MCP, then further features

1. **A minimal browser.** The ADR 0001 ladder, rungs 1–4 as amended by D15:
   toy engine, `html5ever`, hand-written layout, `swash` and `wgpu`, in a
   `winit` window with the D9 human surface. Every rung runs.
2. **MCP.** A second client of the same library API (D10, D27), designed
   against the real engine's output.
3. **Further features.** Engine depth — more of the CSS subset (D12), and the
   context store (D42).

Each stage is a complete, running artifact before the next begins.

The human feature set is **not** widened by this ADR. R5 and D9 still govern:
no human feature is built unless it is needed to debug the engine or is a thin
client over an existing capability. Widening that is a separate ADR.

### D40 — MCP design is parked, except node identity

The [MCP surface document](../design/mcp-surface.md) is parked until stage 2.
Q1 and most of the Q-list are answered better with a working engine than
without one.

**One exception, because it is a one-way door:** DOM nodes carry stable
identities from the first line of the DOM — an arena with ids, not a graph of
`Rc<RefCell<Node>>`. Diff-based reads and handles that survive re-render (the
MCP document's §0 advantages 1 and 2, and Q2) both depend on it. It costs
nothing to write this way the first time and a rewrite to retrofit.

### D41 — D30 is kept; the rest of the sandboxing is deferred

**D30 stands unchanged** — parse, style, layout and paint perform no I/O. It
shapes the code from the first line and pays off immediately: the engine is
testable against fixture files with no network, and in safe Rust the realistic
failure of a hostile page is a panic or a hang in code that cannot reach the
disk. That containment is where the real safety comes from.

Its one visible consequence: when the parser meets `<link rel="stylesheet">` or
`<img>`, it reports what it needs rather than fetching it. The loader fetches;
the engine is handed the results.

**Enforcement is a crate boundary, not a lint.** The engine crate does not
depend on any networking crate. The Dylint candidate in ADR 0005 is dropped.

**Deferred:** D31 (process separation) was already deferred. D32 step 2
(Seatbelt) is deferred to stage 2 at the earliest. D32 step 1 (a separate
macOS user) is unchanged and remains a phase gate in the threat model.

### D42 — MCP and the context store are Cargo features of the application

"Feature" here is the Cargo mechanism: code compiled only when switched on.

```
crates/
  engine/    dom, css, style, layout, paint   — no I/O (D30), no features
  net/       fetching
  browser/   winit + wgpu shell, the D9 surface
             features: mcp, archive            — both off by default
```

The layout is a starting point, not a commitment; the rule it encodes is.
Both features live in the application because both do I/O, which D30 forbids
in the engine.

This **amends D6.** The agent surface and the context store still share fetch
and parse, and still consume the same engine. But *store* and *retrieve* are no
longer part of every build; they are compiled in when wanted. D8's
differentiator survives, because the archive is a feature of the browser with
full access to the engine's structures — not an external extension that must
re-derive them.

Consequences carried with the `archive` feature, and only with it: the
headless-browser capture dependency (D18, T7), and the threats and gates that
concern stored content (T5, T6, T10, T11, O8).

---

## Consequences

- The build order follows ADR 0003 D22 rather than the product frame.
- The first milestone is a window that draws a page, and D24's first
  understanding milestones — the cascade, block and inline layout — come first.
- A default build has no MCP server, no archive, no headless browser, and no
  store on disk. Its threat surface is T1 and T9 only.
- The node-identity requirement enters the DOM before any MCP design exists.
- The MCP document is not discarded. Its open questions stand; they are
  answered in stage 2 against real output.
- The TODO is resequenced to match.
