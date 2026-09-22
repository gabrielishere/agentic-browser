# Session 1: install to control flow

**Date:** 2026-09-22 · **Book:** ch 1, ch 3 (see [plan](plan.md)) · **Code:** `playground/src/main.rs`

## What we did

1. Installed Rust 1.98.1 with `rustup`, just for this user: [setup.md](setup.md)
2. `cargo new playground --vcs none`, then `cargo run`
3. Looked at what the build produces, down to arm64 instructions: [what-cargo-run-builds.md](what-cargo-run-builds.md)
4. Fixed `.gitignore` (`/target/` → `target/`) so every `target/` folder is ignored
5. Worked through variables, types, functions and control flow

## What I learned

| Idea | In one line | Example |
|---|---|---|
| Immutable by default | A variable can't change unless it's `mut` | `let mut year = 2026; year = 2027;` |
| Shadowing | `let` again makes a **new** variable, which can have a new type | `let year = 2026; let year = "next year";` |
| `mut` vs shadowing | `mut` changes the **value**, never the type. Shadowing makes a new variable | assigning `"text"` to a `mut` `i32` won't compile |
| Integer types | Fixed size: `i32` (default), `u8`, `i64`, … `i` = signed, `u` = unsigned, number = bits | `i32` holds about ±2.1 billion |
| Type inference | The compiler works types out; it uses the **default**, not "smallest that fits". A value that doesn't fit is a compile error, not an automatic upgrade | `let big: i64 = 3_000_000_000;` |
| Annotations | `name: Type`, like Python hints, but enforced | `let year: u16 = 2026;` |
| Functions | Parameter types are required; `->` = returns; the last expression **without `;`** is the return value | `fn add_one(year: i32) -> i32 { year + 1 }` |
| Expressions vs statements | A `;` throws the value away and leaves `()` ("unit", Rust's "nothing") | `year + 1;` returns `()`, which is a type error |
| `if` is an expression | It produces a value | `let kind = if y % 4 == 0 { "leap" } else { "normal" };` |
| Ranges | `a..b` excludes `b`; `a..=b` includes it | `for y in 2024..=2028` |
| Conditions must be booleans | No "truthy" numbers like Python's `if count:` | `while countdown > 0` |

## Not tried yet

- The error from `let big = 3_000_000_000;` with no annotation (overflows `i32`)
- The error from `year + 1;` in a function returning `i32`
- Assigning text to a `mut` integer, which gives E0308 mismatched types

Each takes a minute, and reading real compiler errors is a skill in itself.

## Pick up next time

**Book chapter 4: Understanding Ownership.** Fresh session, clear head. See [plan.md](plan.md), Phase A.
