# ADR 0008 — Agents build; the author specifies, reviews, and learns along the way

- **Status:** Accepted
- **Date:** 2026-09-22
- **Deciders:** Gabriel
- **Builds on:** [ADR 0007](0007-build-order-and-optional-features.md) (build order)
- **Amends:** [ADR 0003](0003-learning-is-primary-product-framing-is-instrumental.md) D24 (what "done" means); [ADR 0002](0002-agent-first-browser-with-context-store.md) D15 (what "write" means)
- **Reaffirms:** ADR 0003 D20 (learning is primary), D22 (the decision rule)

---

## Context

The repository now carries an agent-driven build method in `.claude/`: a human
writes a spec and task prompts, an orchestrator agent gates and dispatches each
task to an implementor agent, inspects the result itself, and commits at
verification.

ADR 0003 assumed the author writes the code. D24 states its milestones in that
form — *"I understand how style resolution and the cascade work, **because I
wrote one**"* — and D15's *write the ideas* column (style, layout, paint) was
read as *written by the author*.

Three options were considered for how the method and the learning goal meet:

| Option | Code written by | Learning comes from |
|---|---|---|
| Author writes the ideas; agents write plumbing and failing tests | split | writing |
| **Agents write everything; the author specifies and reviews** | agents | specifying, reviewing, running |
| Agents write a first version; the author rewrites it | both | rewriting |

The author chose the second: *"I'll learn along the way."*

---

## Decisions

### D43 — Agents write the code; the author writes specs and reviews

Implementation is done by the `.claude/` method. The author writes the specs,
prompts and decision records, reviews each cycle's result, and decides what is
merged.

**D15 keeps its line, with a changed meaning.** *Import* still means an external
crate; *write* now means **built in this repository rather than imported** — by
the method, not necessarily by the author's hand. Layout is still not handed to
`taffy`; it is still built here, and the reason it is built here (it is where
the ideas live) is unchanged.

### D44 — "Done" is understanding shown in review, not authorship

D24's milestones are restated. Each one is reached when the author can **explain
the mechanism without the code open** — having specified it, reviewed it, and run
it:

- *I understand the cascade, because I specified one and can say why each rule
  in it resolves the way it does.*
- *I understand box layout, because I can predict what ours does to a fixture
  before running it.*

D20 is unchanged: learning is primary. What moves is the route to it, not its
priority. A cycle that produces working code the author cannot explain has not
met its milestone, whatever its run records say.

### D45 — Work proceeds in development cycles

- **One cycle is one spec**, run by the orchestrator to completion or to a stop.
- **One branch per cycle.** The orchestrator commits onto the cycle branch;
  nothing it produces lands on `main` directly. The author merges.
- **A cycle closes with a review** against its D44 milestone before merging.
  If the approach changed during the cycle, that is an ADR.
- Cycles follow ADR 0007's stages. Cycle 0 adapts the method to this
  repository before any code is built.

---

## Risks

### R8 — Review is shallower than authorship, and will erode

Reading correct code is easier than writing it, and feels like understanding
more often than it is. The failure mode is merging on green run records.

*Mitigation:* D44's test is prediction and explanation, not approval. It is not
enforceable by anything but the author's honesty; it is recorded so that its
erosion is visible as a departure from a written rule rather than as drift.

### R3, revisited

ADR 0001 R3 (*smaller Rust work first*) loses force: Rust fluency is no longer
the gate on progress. It becomes a consequence of reviewing Rust, which is the
bet this ADR makes.

---

## Consequences

- Progress is no longer bounded by the author's Rust. Scope discipline (ADR 0003
  D22, R5) matters more, not less: agents make building cheap, and cheap building
  is the product framing's easiest win.
- Specs become the primary artifact the author writes. Their quality bounds what
  is learned as well as what is built.
- The method's gates and run records are the audit trail of what was built and
  how well it was evidenced; D44 reviews are the record of what was learned.
