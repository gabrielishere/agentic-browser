# ADR 0004 — Preserve optionality toward product, without adopting product framing

- **Status:** Accepted. D25 confirmed 2026-09-20; D29 resolved 2026-09-20.
- **Date:** 2026-09-20
- **Deciders:** Gabriel
- **Builds on:** [ADR 0003](0003-learning-is-primary-product-framing-is-instrumental.md)
- **Reaffirms:** ADR 0001 D1–D4, ADR 0003 D20

---

## Context

The core technology, if built, could later be extended into a product. The
question is whether to record that now.

**This ADR does not plan a product.** It describes no users, no market, no
roadmap, and makes no commitment to ship. ADR 0003 D20 stands: this is a
part-time learning exercise and success is measured as understanding.

What it does instead is narrower and nearly free. A small number of decisions
are **cheap now and expensive or impossible later**. Making them well does not
commit the project to anything; failing to make them forecloses options
silently. The distinction is between *deciding to build a product* and
*declining to prevent one* — only the second is in scope here.

Five such decisions were identified. Everything not on this list is out of
scope by design; the list is short on purpose.

> **Standing caution.** ADR 0003's closing consequence warned that the product
> framing is seductive because it produces visible progress, and would keep
> trying to become primary. This ADR is the nearest that pressure may
> legitimately come. If a future ADR proposes work justified by product value
> rather than learning value, it is the drift 0003 named, and 0003 D22 governs.

---

## Decisions

### D25 — License: `MIT OR Apache-2.0` ~~*(confirm before first publish)*~~ *(confirmed 2026-09-20)*

The Rust ecosystem convention, used by the overwhelming majority of crates.
Permissive, trusted by companies, and Apache-2.0 carries an explicit patent
grant that MIT lacks. Keeps every downstream option open.

**This is the one-way door.** Once outside contributions are accepted without a
contributor agreement, relicensing requires permission from every contributor,
which in practice means never. The decision is effectively free today and
effectively impossible in two years.

**Alternative considered: GPL-3.0 plus a CLA.** Copyleft with centralised
copyright preserves the dual-licensing play (the Qt and MySQL model) — sell
exceptions to commercial users while the project stays open. It buys commercial
leverage that permissive licensing does not. It costs contributor friction,
some contributors decline CLAs on principle, it signals commercial intent this
project explicitly disclaims, and it makes the work unusable as a dependency by
most companies. Rejected on balance: the leverage it preserves is only valuable
under the product framing ADR 0003 declines to adopt, and the friction is
immediate while the leverage is hypothetical.

**Relevant precedent:** Zed splits deliberately — GPL-3.0-or-later for
application crates, Apache-2.0 for reusable library crates (`gpui`,
`sum_tree`, `util` and ~30 others), specifically so the reusable parts stay
reusable. That split is available here too if the application and library
answers should ever differ. It is not adopted now because there is no
application yet.

> **Confirmed 2026-09-20.** `MIT OR Apache-2.0` adopted deliberately.
> `LICENSE-MIT` and `LICENSE-APACHE` are in the repository root, and the
> inbound-equals-outbound statement required by D26 is in `README.md`.

### D26 — Contribution terms: inbound equals outbound

Contributions are licensed under the same terms as the project. Stated plainly
in `README.md` and `CONTRIBUTING.md` when one exists. No CLA.

This is the standard, low-friction arrangement, and it follows from D25. It is
recorded separately because it is the mechanism by which D25 becomes
irreversible — the licence and the contribution terms are one decision wearing
two hats.

### D27 — Core is a library; the application is a thin shell

The engine is a crate with a clean public API. The runnable browser is a
wrapper over it.

Justification is architectural before it is optional-value: ADR 0002 D10 already
requires the human UI to be a client of the same internal API the MCP server
uses. A library core is how that is achieved rather than merely intended. The
optionality — that it could later be embedded, shipped differently, or reused —
comes free as a consequence.

Cost: none. This is better structure regardless, and it makes testing easier.

### D28 — The archive format is versioned and documented from the first write

Every stored record carries a format version. The format is documented
alongside the code, not reconstructed from it.

Without this, stored data can never be migrated safely, which forecloses ever
giving the thing to anyone — and also makes the author's own data disposable
across refactors, which will hurt long before any hypothetical user appears.

Cost: one integer field and a paragraph of documentation, paid once.

### D29 — The name stays deferred, with criteria recorded

> **Resolved 2026-09-20: the name is `agentic-browser`.** O7 is closed. The
> deferral ended when a repository was created, which is the point at which a
> name stops being optional. It satisfies the criteria below except the first —
> see the note after them.

~~O7 remains open.~~ *(Closed — see above.)* ADR 0002's placeholder reasoning stands: naming before the
thing exists names the wrong thing.

Criteria to apply when deciding, recorded now so the decision is quick later:

- `crates.io` name available — Rust names are first-come, and short ones are gone
- GitHub org or repository available
- No collision with anything large enough to bury it in search
- Pronounceable and typeable
- Not named after a protocol. MCP is roughly two years old; naming after an
  integration dates the project and describes the wrong layer

> **Note on the first criterion.** `agentic-browser` is descriptive rather than
> distinctive, so the GitHub path is namespaced by the account and fine, but
> `crates.io` is a flat namespace and the name may well be taken or contested
> there. This does not matter until something is published. If a published
> crate needs a distinctive name later, the repository name and the crate name
> are allowed to differ — D27's library/application split makes that
> straightforward.

---

## Consequences

- Four decisions are settled at near-zero cost. One (D25) needs a deliberate
  yes. *(Given 2026-09-20.)*
- None of the above obliges the project to become a product, and ADR 0003 D20
  continues to govern.
- If a product is ever pursued, that is a change of kind and needs its own ADR
  — this one only ensures the option still exists at that point.
- The list is deliberately closed. Further "but what if it's a product"
  decisions should be refused unless they are genuinely one-way doors, which
  almost none are.
