# TODO

**Last updated:** 2026-10-27

> Sequenced work, not an exhaustive list. Open *questions* live in the documents
> that own them — this file points at them rather than restating them, so the
> two cannot drift apart.

---

## Resuming cold

Read in this order:

1. [`README.md`](../README.md) — what this is
2. [ADR 0002](adr/0002-agent-first-browser-with-context-store.md) — purpose and architecture
3. [ADR 0007](adr/0007-build-order-and-optional-features.md) — build order: what comes first and why
4. [ADR 0005](adr/0005-sandboxing-boundary-and-io-free-engine-core.md) D30 — the one structural rule the engine must obey
5. [ADR 0008](adr/0008-agents-build-author-specifies-and-reviews.md) — agents build in cycles; the author specifies, reviews and learns
6. [Rust learning plan](learning/rust/plan.md) — how learning fits each cycle; [concepts.csv](learning/rust/concepts.csv) tracks what has been covered

Everything decided is in `docs/adr/`, D1–D45, contiguous. Everything undecided
is a `Q` in the MCP design document, an `S` in the security design, or an `O`
in an ADR.

---

## First — make the build pipeline safe

Agents build the code (ADR 0008), so T12 is live before any browser code
exists. Answer [`design/security.md`](design/security.md) **S1–S3 and S9**
(machine isolation, agent permissions, dependency approval, a gate before
Cycle 0) before the first task is dispatched.

Background for those answers: items 11–16 of the
[security reading list](learning/security/reading-list.md), the build-pipeline
section. The rest of the list waits for the gate before rung 4.

Then **Cycle 0**: adapt the `.claude/` method to this repository (ADR 0008
D45). No Rust is built in it.

---

## Now — stage 1: a minimal browser (ADR 0007 D39)

The ADR 0001 ladder, rungs 1–4. Every rung is a program that runs.

**Rust is learned alongside the build, not before it** (from 2026-10-16): each
cycle's review includes a learning session on the Rust it used. See the
[plan](learning/rust/plan.md). ADR 0001 R3 still holds; it is met cycle by
cycle, against D44's bar.

### 1. Workspace skeleton

`engine`, `net` and `browser` crates per ADR 0007 D42. `engine` depends on no
networking crate — that is how D30 is enforced.

### 2. Rung 1 — toy engine

HTML subset → DOM, CSS subset, box layout, paint to a window.

- **DOM nodes carry stable ids from the first line** — an arena, not
  `Rc<RefCell<Node>>` (ADR 0007 D40). The only MCP concern allowed into stage 1.
- The parser reports stylesheets and images it needs; it does not fetch them
  (D30, as spelled out in D41).
- Test against HTML fixture files in the repo, not live pages.

### 3. Rung 2 — real parsers

Swap in `html5ever`. Decide whether CSS parsing is imported (`cssparser`) or
written — D15 does not say.

### 4. Rung 3 — real layout, written by hand

Block and inline flow, the box model, the D12 subset. D24's milestone:
*I understand box layout, because mine handles block and inline flow.*

### 5. Rung 4 — real text and paint

`swash` for shaping, `wgpu` for the GPU, `winit` for the window. The D9 human
surface: URL bar, back and forward, viewport, find-in-page.

**Gate before step 5 touches real websites:** T1 mitigated and D32 step 1 (a
separate macOS user) in place — see the [threat model](security/threat-model.md).

---

## Parked — stage 2: MCP (ADR 0007 D40)

Resume with a working engine. The plan as it stood, kept so nothing is lost:

1. [`design/mcp-surface.md`](design/mcp-surface.md) §1 — the read representation
   (**Q1**). Method: pick one awkward real page and design the ideal `read`
   for it, now against the engine's actual output.
2. §1.1 prior art, §1.2 node identity (**Q2** — the arena from stage 1 is the
   starting point), §1.3 diffs.
3. §2 per-tool signatures for `open` and `read`, and the error shape (**Q7**).
4. §5 worked transcripts — the evaluation method. **Q3** is decided here.
5. §6 — convert to a pointer to the threat model.

Built as the `mcp` Cargo feature. T2 gates the first `open(url)`.

## Parked — stage 3: further features

Engine depth — more of the D12 CSS subset — and the context store as the
`archive` Cargo feature (ADR 0007 D42). Human features beyond D9 stay out
unless a new ADR revisits R5.

---

## Blocked or deferred

| Item | Blocked on |
|---|---|
| **O8** — is a domain denylist mandatory? | Gated: must be answered *before the first snapshot is saved*. T5 is critical |
| **O9** — retention policy | Nothing bounds archive growth |
| **O10** — is pre-JS HTML always stored? | Confirm against real storage cost |
| **Q4** — shared or isolated session state | Only matters once cookies or logins exist |
| **T11** — stored scripts | Gated: must be mitigated *before Phase 2 begins* |
| Anything `act`-shaped | T4 needs a mechanism, not vigilance |

O8–O10 and T11 only arise with the `archive` feature.

---

## Housekeeping

- [ ] GitHub → Settings → Emails → **Block command line pushes that expose my email**
- [x] Delete `/tmp/agentic-browser-git-backup-*` (gone as of 2026-10-27)
- [ ] Decide whether `docs/design/mcp-surface.md` §4 survives, given statelessness is settled
