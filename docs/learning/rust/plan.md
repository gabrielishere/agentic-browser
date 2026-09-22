# Rust learning plan

**Started:** 2026-09-22 · **Updated:** 2026-09-22

> Learning Rust well enough to **review** what the agents build ([ADR 0008](../../adr/0008-agents-build-author-specifies-and-reviews.md)
> D44): explain the mechanism without the code open. Each phase ends with a
> milestone stated that way, and each phase is tied to the build cycle it prepares
> you to review.
>
> Worked hands-on in `playground/`, one step at a time. Session notes are in this
> directory (`session-NN.md`).

---

## Sources

The main text is **The Rust Programming Language** ("the Book") by Steve Klabnik,
Carol Nichols and Chris Krycho, with contributions from the Rust community.

| Source | Online | Local copy (installed with Rust) |
|---|---|---|
| **The Book** | [doc.rust-lang.org/book](https://doc.rust-lang.org/book/) | `rustup doc --book` |
| Rust by Example | [doc.rust-lang.org/rust-by-example](https://doc.rust-lang.org/rust-by-example/) | `rustup doc --rust-by-example` |
| Standard library docs | [doc.rust-lang.org/std](https://doc.rust-lang.org/std/) | `rustup doc --std` |
| The Rust Reference (the precise rules) | [doc.rust-lang.org/reference](https://doc.rust-lang.org/reference/) | `rustup doc --reference` |
| Rustlings: small exercises that follow the Book | [github.com/rust-lang/rustlings](https://github.com/rust-lang/rustlings) | not installed |
| robinson: a toy browser engine in Rust | [github.com/mbrubeck/robinson](https://github.com/mbrubeck/robinson) | see [prior art](../../research/prior-art.md) |

**Version match.** The online Book targets Rust 1.90 or later and the 2024 edition. The
local copy matches the installed Rust (1.98.1). The playground uses `edition = "2024"`.

**How session 1 was taught.** The steps followed the order of topics in Book chapter 3,
but were written from general knowledge, not taken from the Book's text. Read the
chapter itself to check them.

---

## Phases

Chapter numbers and titles are taken from the local copy of the Book (Rust 1.98.1).

### Phase A: Foundations  ← **current**

| Ch | Title | Status |
|---|---|---|
| 1 | Getting Started | ✓ (session 1: install, `cargo new`, `cargo run`) |
| 2 | Programming a Guessing Game | **skipped for now.** It adds the `rand` crate, the first dependency. Do it deliberately: it's the first time a crate's build scripts run on this machine (T9) |
| 3 | Common Programming Concepts | ✓ (session 1: variables, `mut`, shadowing, types, functions, control flow). Not yet covered: 3.2's compound types (tuples, arrays) and 3.4 comments |
| **4** | **Understanding Ownership** | **next.** The idea that makes Rust different |

**Milestone:** *I can explain why this doesn't compile, and fix it three different ways:*
using a value after it has been moved.

**Practice:** Rustlings sections `variables`, `functions`, `if`, `primitive_types`,
`move_semantics`.

### Phase B: Modelling data

| Ch | Title | Why it matters for the browser |
|---|---|---|
| 5 | Using Structs to Structure Related Data | A box in layout is a struct |
| 6 | Enums and Pattern Matching | A DOM node is an enum: element, text, comment |
| 8 | Common Collections | `Vec`, `String`, `HashMap`: the DOM arena is a `Vec` |
| 9 | Error Handling | Parsers fail; `Result` is how Rust says so |

**Milestone:** *I can model a tiny DOM (elements with children, and text) with
structs and enums, and explain each type choice.*

**Prepares you to review:** Cycle 2 (HTML subset → DOM arena).

### Phase C: Organising code

| Ch | Title | Why it matters |
|---|---|---|
| 7 | Packages, Crates, and Modules | The `engine` / `net` / `browser` workspace (ADR 0007 D42) |
| 10 | Generic Types, Traits, and Lifetimes | Lifetimes are ownership over time; unavoidable in parsers |
| 11 | Writing Automated Tests | Every cycle's evidence is tests |

**Milestone:** *I can read a crate's module tree and a test, and say what the test
proves and what it doesn't.*

**Prepares you to review:** Cycle 1 (workspace and fixture harness).

### Phase D: Bridge to the browser

| Item | Why |
|---|---|
| Ch 13: Functional Language Features: Iterators and Closures | Tree walks in style and layout |
| Ch 15: Smart Pointers | `Rc<RefCell<…>>`, and **why ADR 0007 D40 chose an arena instead** |
| robinson, parts 1–7 | Retype the DOM and parser in the playground, then compare with the agents' version |
| [Web Browser Engineering](https://browser.engineering/), early chapters | The same pipeline explained in Python, which you already know |

**Milestone:** *I can predict what the layout code does to a fixture before running
it* (ADR 0008 D44).

**Prepares you to review:** Cycles 3–6 (CSS, style, layout, paint).

### Phase E: As the project needs it

| Ch | Title | When |
|---|---|---|
| 12 | An I/O Project: Building a Command Line Program | Any time; a good consolidation exercise |
| 14 | More about Cargo and Crates.io | Before approving dependencies (security S3) |
| 16 | Fearless Concurrency | When layout or networking goes parallel |
| 17 | Fundamentals of Asynchronous Programming | The `net` crate |
| 18–20 | OOP features, Patterns and Matching, Advanced Features | Reference, as needed |
| 21 | Final Project: Building a Multithreaded Web Server | Optional; close to the MCP server in stage 2 |

---

## Loose ends

- The `aarch64-apple-darwin` follow-ups in [setup.md](setup.md) and
  [what-cargo-run-builds.md](what-cargo-run-builds.md).
- Book chapter 2 (the first external crate). Decide consciously; see Phase A.
