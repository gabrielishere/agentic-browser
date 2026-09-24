# Session 2: compiler errors, ownership, and a stack/heap detour

**Date:** 2026-09-24 · **Book:** ch 4 (see [plan](plan.md)) · **Code:** `playground/src/main.rs`

## What we did

1. Deliberately triggered three compiler errors left over from session 1's "not tried
   yet" list: an integer literal too big for `i32`, a semicolon that throws away a
   function's return value, and text assigned to a `mut` integer.
2. A detour into the stack and heap: `std::mem::size_of`, heap addresses via
   `.as_ptr()`, and ASLR (the same binary printing a different stack address on every
   run).
3. A detour into target triples: `aarch64-apple-darwin`, cross-compiling, and that
   `cargo run` silently targets the host machine by default.
4. Book ch 4: ownership. Triggered the classic moved-value error, then fixed it three
   different ways.
5. References and borrowing: the one-`&mut`-or-many-`&`, never-both rule.

## What I learned

| Idea | In one line | Example |
|---|---|---|
| Overflow is a compile error | A literal too big for its inferred type doesn't upgrade automatically | `let big = 3_000_000_000;` → `i32` can't hold it; compiler suggests `u32` |
| `mut` is not a type escape hatch | `mut` allows a new *value* of the same type, never a different type | `let mut age = 30; age = "thirty";` → E0308, same error family as ownership |
| Move vs. copy | Assigning a `Copy` type (`i32`) copies it; assigning anything else (`String`) moves it, and the old name becomes invalid | `let s2 = s1;` — `s1` is unusable afterward if it's a `String` |
| Why moves exist | A `String` is a stack header (pointer, length, capacity) pointing at heap bytes. If both `s1` and `s2` stayed valid, both would try to free the same heap memory when they go out of scope | double free, prevented at compile time |
| Fixing a moved-value error, three ways | Clone it (real second allocation), borrow it (`&`, never take ownership), or just stop using the old name once you don't need it | `s1.clone()` / `&s1` / rename the later use to `s2` |
| Stack vs heap | Every variable's name lives on the stack. Only the *value* differs: fixed-size types are stack-only; growable types (`String`, `Vec`) keep a small header on the stack pointing at heap data | `size_of::<i32>()` = 4, `size_of::<String>()` = 24, regardless of string length |
| ASLR | The compiler fixes a variable's *offset* within a stack frame at compile time; the OS randomizes *where the frame itself* sits at runtime, as a security measure | same binary, three runs, three different addresses, same low bits |
| Target triple | `architecture-vendor-os`, e.g. `aarch64-apple-darwin`. Compiled output is tied to one triple, not portable across it | a Mach-O arm64 binary won't run on Linux or an Intel Mac without cross-compiling |
| Borrowing rule | Many read-only (`&`) references, or exactly one mutable (`&mut`) reference — never both live at once | `let r1 = &s; let r2 = &mut s;` while `r1` is still used later → E0502 |
| Why the borrowing rule exists | Stops a reader from observing a value mid-write, at compile time rather than as an occasional runtime bug | E0502 fires even though nothing here would have crashed at runtime — it's the *possibility* that's banned |

## Not tried yet

- Ch 3.2 (tuples, arrays) and 3.4 (comments) — still outstanding from session 1.
- Ch 4.3, slices.
- Ch 2, the guessing game (`rand`, the first external dependency) — still parked.

## Pick up next time

**Book chapter 4.3: Slices**, finishing ch 4, then **Phase B: structs and enums**
(ch 5–6) — see [plan.md](plan.md). Fresh session, clear head.
