# ADR 0003 — Learning is primary; the product framing is instrumental

- **Status:** Accepted
- **Date:** 2026-09-20
- **Deciders:** Gabriel
- **Builds on:** [ADR 0001](0001-browser-as-rust-learning-vehicle.md), [ADR 0002](0002-agent-first-browser-with-context-store.md)
- **Resolves:** drift between 0001 and 0002 over what this project *is*

---

## Context

ADR 0001 and ADR 0002 describe two different projects, and the contradiction is
explicit in the text.

**ADR 0001** says it is a *"part-time personal learning exercise, **not a
product commitment**"* (D1) and that *"success is measured as
**understanding, not shipping**"* (D3). It says the browser is a *"north
star, not the next task"* (D2), and R3 adds *"smaller Rust work first."*

**ADR 0002** says *"Phase 1 is a complete and useful **product** without
solving the hardest problem in the space"* (Consequences), sets a rule about
what *"ships"* (R5), analyses prior art and differentiators, and lays out a
two-phase build plan.

Those are not compatible as written. One says there is no product commitment;
the other names Phase 1 a product.

This matters because the two framings give opposite answers to real questions:

| Question | Learning frame | Product frame |
|---|---|---|
| Write layout or import `taffy`? | Write it — that's the education | Import it — ship sooner |
| Build renderer or MCP surface first? | Renderer — it's where the systems learning is | MCP — it's the differentiator |
| Is a browser that never renders a real site a failure? | No | Yes |
| What justifies Phase 2 (JavaScript)? | Curiosity | User need |

Left unresolved, whichever framing feels most compelling on a given evening
decides the build order. That is the actual risk — not that either framing is
wrong, but that the project silently alternates between them.

---

## Decision

### D20 — Learning is primary

ADR 0001 stands unamended on this point. This is a part-time personal learning
exercise. Success is measured as understanding. There is no delivery
obligation, no user to disappoint, and no date.

### D21 — The product framing is a device, not a commitment

The opinionated product shape introduced in ADR 0002 — agent-first, context
store, MCP surface — exists because **ADR 0001 R2 has no other solution.**
Scope is unbounded by default, and an opinion about what the thing is *for* is
the only mechanism that bounds it. "Build a browser" is unfinishable; "build
the browser described in ADR 0002" is merely very hard.

So the product framing is retained in full. It sets direction and it sets a
quality bar — build as though it were a product, because that is what makes the
learning real rather than academic. But it is instrumental. It is scaffolding
for the learning, not a promise to anyone.

### D22 — Decision rule when the two conflict

1. **Default: understanding wins.** Where a choice is between *faster to
   working software* and *more understanding*, choose understanding. This is
   what makes ADR 0002 D15 ("import the spec-grinding, write the ideas")
   coherent rather than arbitrary.
2. **Exception: scope discipline wins.** Where the product opinion is what
   bounds scope, it overrides — because unbounded scope kills the learning too
   (ADR 0001 R1, R2). If something is not needed by the browser ADR 0002
   describes, it is not built, however interesting.

Stated shortly: **the product opinion decides *what* is in scope; the learning
goal decides *how* it is built.**

### D23 — Corrections to language in ADR 0002

- *"Phase 1 is a complete and useful **product**"* → read as **"a complete and
  useful artifact."** No product commitment is implied.
- **Phase 1 and Phase 2 describe capability order, not a roadmap.** They are
  the sequence in which things would be built *if and when* building starts.
  They do not supersede ADR 0001's "north star, not the next task," which
  stands: substantial Rust competence comes first.
- R5's *"ships"* → read as **"is built."**

### D24 — What "done" means, concretely

Since success is understanding, milestones are stated as understanding.
Completable examples:

- *I understand how style resolution and the cascade work, because I wrote one.*
- *I understand box layout, because mine handles block and inline flow.*
- *I understand why browsers separate layout from paint, because I hit the
  reason.*
- *I understand what an agent actually needs from a page, because I designed a
  surface and used it.*

Not milestones: *I built a browser.* *It renders Hacker News.* Those may
happen, and would be pleasing, but they are not the measure.

---

## Consequences

- The build order follows from D22 rather than from mood. When in doubt, the
  slower and more educational path is correct.
- ADR 0002's product content is preserved and remains authoritative for
  **scope** — what is in and out.
- If this project is ever published and attracts users, that would be a change
  of kind, not of degree, and would need a new ADR. Users create obligations
  that D20 explicitly disclaims.
- The reverse drift is now the one to watch: the product framing is seductive
  precisely because it produces visible progress, and it will keep trying to
  become primary.
