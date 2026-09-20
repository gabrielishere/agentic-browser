# ADR 0001 — An open-source browser project as the vehicle for learning Rust

- **Status:** Accepted
- **Date:** 2026-09-20
- **Deciders:** Gabriel
- **Supersedes:** —
- **Superseded in part by:** [ADR 0002](0002-agent-first-browser-with-context-store.md) — see the note on the ladder below.
- **Reaffirmed by:** [ADR 0003](0003-learning-is-primary-product-framing-is-instrumental.md) — D1–D3 below stand unamended and take priority over 0002's product language.
- **Next:** Browser design (scope, opinion, architecture) — resolved in ADR 0002.

---

## Context

### Starting position

An AI engineer, competent full-stack, with no Rust and no systems or low-level
computing background. Zed is already the daily driver for agentic engineering.
Its responsiveness prompted the question that started this: *why is it so
fast?*

Three stated goals: learn Rust, understand what makes software fast, build
custom applications.

Hardware: MacBook Neo (`Mac17,5`), Apple A18 Pro, 6 cores (2 performance + 4
efficiency), 8 GB unified memory, macOS Tahoe 26.5.1.

### What we established about Zed's performance

The question "why is Zed fast" resolved into two distinct answers that are
often conflated:

**Rust sets the floor.** No garbage collector, so no unpredictable pauses —
and "snappy" is mostly about worst-case frame time, not average speed. Rust's
ownership model also makes Zed's heavy background concurrency (syntax parsing,
LSP, git, project search, collaboration sync) tractable to build and maintain.

**Architecture sets the ceiling.** Rust is necessary but not sufficient.
Sublime Text is C++ and comparably fast; a slow editor in Rust is entirely
possible. Zed is fast because it does *less work*: `crates/sum_tree/` (a
concurrency-friendly B-tree) makes edits, folds, wrapping and scrolling
O(log n); `crates/rope/` means editing a large file never copies it; and
`crates/gpui/` puts rendering on the GPU.

Historical support: the same team built Atom on Electron, found the ceiling
too low, and responded by changing *both* the language and the architecture.

### Why a browser is well-matched

**Rust has a strong claim on this problem.** Mozilla created Rust
substantially in order to build Servo. Servo components shipped into Firefox
in 2017 (Stylo for CSS, WebRender for rendering). Chromium is now adopting
Rust for parsing and security-sensitive components.

**The ecosystem exists, and Zed already uses parts of it.** Observed in this
workspace:

| Crate | Where | Role |
|---|---|---|
| `taffy = "=0.13.0"` | `crates/gpui/Cargo.toml:94` | flexbox/grid layout |
| `html5ever = "0.39"` | `Cargo.toml:638` | HTML parser (Servo lineage) |
| `url = "2.2"` | `Cargo.toml:904` | URL parser (Servo lineage) |
| `swash = "0.2.6"` | `Cargo.toml:848` | text shaping and rasterization |
| `wgpu = "29.0.4"` | `Cargo.toml:929` | GPU rendering |
| `taffy = { opt-level = 3 }` | `Cargo.toml:1064` | layout optimised even in dev builds |

**GPUI is structurally the lower half of a browser rendering engine** — element
tree, flexbox layout via Taffy, Tailwind-like styling, GPU paint, text shaping,
event dispatch through a tree. Strip the web platform from a browser and what
remains closely resembles GPUI. The codebase already in daily use is an
unusually good reference implementation for the ambition.

**Electron is the inverse move, and is not implicated.** Electron *consumes* a
rendering engine (Chromium) to ship web tech as a desktop app. Building a
browser means building that engine — upstream of Electron, on the same side of
the line as GPUI. Electron's value proposition is hiding exactly the layers
this project exists to learn.

### Learning surface

A browser touches, in one coherent artifact: parsing and compilers (tokenizers,
grammars, ASTs, HTML error recovery); tree data structures and the DOM; layout
algorithms and constraint solving; GPU programming (shaders, rasterization,
compositing); text rendering (font formats, shaping, bidirectional text,
Unicode); networking (HTTP, TLS, pooling, caching); concurrency and event
loops; sandboxing and same-origin security; memory behaviour and profiling.

This maps closely onto the stated gap.

---

## Decision

**An open-source browser project is affirmed as compatible with — and a good
vehicle for — the Rust goal.**

Qualified as follows:

### D1 — A part-time personal learning exercise, not a product commitment

### D2 — A north star, not the next task

It gives direction to smaller projects rather than being the first one.

### D3 — Success is measured as understanding, not shipping

### D4 — Open source is a deliberate property of the project, not incidental

### Structure: a ladder where every rung runs

1. Toy engine — HTML subset → DOM, CSS subset, box layout, paint to a window.
2. Real parsers — swap in `html5ever`, `cssparser`.
3. Real layout — hand off to `taffy`.
4. Real text and paint — `swash` for shaping, `wgpu` for GPU.
5. JavaScript — embed an existing engine rather than writing one.

The property that matters: at no point is there an unfinished browser in hand,
only a progressively less toy one.

> **Partly superseded by ADR 0002.** Step 3 ("hand off to `taffy`") no longer
> holds: ADR 0002 D15 establishes *import the spec-grinding, write the ideas*,
> under which layout is written rather than imported, because it is the single
> best thing in the stack to understand deeply. Steps 2, 4 and 5 stand. The
> ladder's shape — every rung is a program that runs — is unaffected.

---

## Flagged risks and open items

### R1 — No natural finish line *(primary risk)*

"Build a browser" has no completion state. The failure mode is not difficulty;
it is grinding part-time for months without arriving anywhere, and quiet death.

*Mitigation:* the ladder above, plus understanding-based milestones. "I now
know how layout works" is completable. "I built a browser" is not.

### R2 — Scope is effectively unbounded

Full web compatibility is a multi-hundred-person-year problem. Chromium is
~35M lines. Servo has had 13 years and hundreds of contributors and is not a
daily driver. The hard parts are not where beginners expect: CSS layout
(floats, tables, grid), text shaping (bidi, ligatures, complex scripts), and
HTML error recovery are each brutal; a JavaScript engine is a career.

*Mitigation:* scope by opinion — see O1.

### R3 — Sequencing

Rust's learning curve is real, and browser-engine code will read as noise
before the basics land. Starting at the browser risks early discouragement.

*Mitigation:* north star, not first step. Smaller Rust work first, chosen
because it serves the browser.

### R4 — Hardware ceiling on outlier workspaces

<!-- Heading clarified after this ADR moved out of the Zed checkout, where
     "this specific repo" was unambiguous and no longer is. -->


8 GB and 2 performance cores is fine for a toy engine and for ordinary Rust
work. It is strained only by outlier workspaces — Zed's own 264-crate build,
or Servo. Constraint is memory, not disk (~357 GB free).

*Mitigation:* work per-crate rather than `--workspace`; prefer `cargo check`
for the edit loop; cap parallelism (`-j 2`) when building something large.

### O1 — Open question: what is the browser *for*?

This is the most important unresolved item and the one that governs scope.

- **General-purpose** → a *compatibility* problem. Racing a 35M-line spec
  surface; every unit of effort spent matching what already exists.
- **Opinionated** → a *design* problem. More interesting and more reachable,
  because deciding what not to support is the only real way to make a browser
  tractable.

Candidate opinions: agentic (the browser as an agent surface rather than a
human one); local-first; reader-focused (drop most of the web platform on
purpose); dev-tool-first (the inspector is the product).

*Noted preference:* the agentic framing is the one where existing AI
engineering expertise compounds rather than sitting idle — the only axis on
which this is not a beginner project, and the version most likely to produce
something other people want rather than only something learned from.

**Deferred to the next discussion: browser design.**

> **Resolved by [ADR 0002](0002-agent-first-browser-with-context-store.md) D5.**
> Note that 0002 did not pick one of the four candidate opinions above — it
> **fused two of them**. Agent-first and local-first turned out to be one
> substrate rather than competing options, because an agent surface and a
> personal archive need the same pipeline (fetch → parse → store → retrieve).
> The four candidates should therefore be read as a starting palette, not a
> menu.

---

## Consequences

- Rust learning gains a concrete direction; exercises can be chosen for whether
  they serve the browser rather than picked at random.
- Zed shifts from "tool I use" to "reference implementation I read" —
  `crates/sum_tree/`, `crates/rope/`, `crates/gpui/` become study material with
  a purpose.
- The project is public by decision, which raises the bar on legibility and
  invites outside input.
- Scope discipline becomes the standing risk to manage, indefinitely.
