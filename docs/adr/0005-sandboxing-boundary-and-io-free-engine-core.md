# ADR 0005 — The sandboxing boundary and an I/O-free engine core

- **Status:** Accepted
- **Date:** 2026-09-20
- **Deciders:** Gabriel
- **Builds on:** [ADR 0004](0004-preserve-optionality-toward-product.md) D27 (library core)
- **Related:** [threat model](../security/threat-model.md) T1, T2, T7, T8

---

## Context

This project feeds deliberately hostile input to parsers written by someone
learning the language. That is not a criticism of the plan; it is simply what a
browser is.

**Development is the high-risk period, not the low-risk one.** The usual
instinct — harden before release — is backwards here. During development the
parser is incomplete, the error paths are unwritten, and the thing is being
pointed at real websites on the author's only machine. There is no blast
radius, because the blast radius is the laptop.

Real browsers answer this with **process separation**: the code that parses
hostile input runs in a restricted process with no disk or network access,
while privileged work happens elsewhere. Chromium spent years retrofitting
this, which is the relevant warning — a single-process monolith whose parser
freely opens files and sockets cannot be sandboxed later without a rewrite.

But building multi-process now would likely sink the project. IPC,
serialisation and process lifecycle management are hard in any language, and
harder in one being learned. ADR 0003 D22 applies: scope discipline wins where
unbounded scope would kill the learning.

The resolution separates the one-way door from the reversible tactic.

**Reference implementation available:** Zed's `crates/sandbox/` implements
cross-platform sandboxing in Rust — `macos_seatbelt.rs`, `linux_bubblewrap.rs`,
`windows_wsl.rs`. Per D16 this is reading material, not a dependency.

---

## Decisions

### D30 — The engine core performs no I/O

Parsing, style resolution, layout and paint take data in and return structures
out. They do not open files, make network requests, spawn processes, read
environment variables, or touch the archive. Input is handed to them; output is
taken from them.

**This is the one-way door.** It costs nothing today — it is better structure
regardless, it makes the core trivially testable with fixture inputs, and it is
close to what D27 already requires. It costs a rewrite if deferred.

Consequence for code review of one's own work: any `std::fs`, `std::net`,
`std::process` or `std::env` import appearing inside the engine core is a
defect, not a convenience.

### D31 — Process separation is deferred, not foreclosed

Multi-process architecture is not built now. D30 is what keeps it available: a
core that performs no I/O can later be moved behind a process boundary as a
refactor rather than a rewrite.

Revisit when the engine works and the author's Rust is no longer the limiting
factor. Not before.

### D32 — Development-time sandboxing, by cost

Applied in this order, each adopted when the preceding one stops being
sufficient:

1. **A separate macOS user account**, from the moment the engine is first
   pointed at real websites. The process cannot read the author's home
   directory. Roughly ten minutes of setup, and it directly addresses the
   stated concern — that a mistake should not reach personal files. It does
   *not* address network egress or resource exhaustion.
2. **Seatbelt via `sandbox-exec`**, once there is enough engine to be worth
   sandboxing properly. Filesystem and network restriction by profile.
   Deprecated by Apple but functional; Chromium used it for years and Zed uses
   it today.
3. **Container or VM** — considered and **not** adopted for routine
   development. On macOS a container implies a Linux VM, which fits a GUI
   application poorly; a full VM would consume around half of this machine's
   8 GB. Revisit if the hardware changes.

---

## Consequences

- The expensive architectural decision is made while it is still free.
- The engine core becomes testable from fixture files with no environment
  setup, which is worth having on its own merits.
- Sandboxing stops being a thing to do "later" and becomes a property the
  architecture already has.
- Phase gates in the threat model can now reference a real boundary rather than
  an intention.
- **D30 is only true if it stays true.** It is one careless `std::fs::read` from
  being false, and nothing enforces it automatically. Candidate future
  enforcement: a Dylint lint, or confining the core to its own crate with
  dependencies that make I/O unavailable.
