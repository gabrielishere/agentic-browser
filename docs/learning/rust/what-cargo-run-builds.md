# What `cargo run` builds

**Explored:** 2026-09-22 · playground "Hello, world!" · Rust 1.98.1, `aarch64-apple-darwin`

## The files in a new project

Three are **yours** and committed:

| File | What it is |
|---|---|
| `Cargo.toml` | Your settings: name, edition, dependencies |
| `src/main.rs` | Your code |
| `Cargo.lock` | Cargo's record of the exact versions it resolved. Generated, but committed so builds are repeatable |

Everything in `target/` is generated and disposable (`cargo clean`, and it's gitignored):

| Item | What it is |
|---|---|
| `debug/playground` | **The program.** `Mach-O 64-bit executable arm64`, 462 KB, runs without Rust installed |
| `debug/deps/*.rcgu.o` | Object files: machine code with gaps still to be filled by the linker |
| `debug/deps/*.rmeta` | Summary of the crate's types, used by other crates (empty here, nothing uses it) |
| `debug/*.d` | Which source files a build depends on, so Cargo knows when to rebuild |
| `debug/incremental/`, `.fingerprint/` | Remembers what's compiled; changing one line doesn't rebuild everything |
| `debug/build/` | Output of crates' build scripts. Empty with no dependencies (build scripts are the supply-chain risk, T9) |
| `.cargo-*lock`, `CACHEDIR.TAG`, `.rustc_info.json` | Locks, a "this is a cache" marker for backup tools, the compiler version |
| `flycheck0/` | **Made by the editor, not `cargo run`.** rust-analyzer runs `cargo check` in the background |

`dev` profile = unoptimised + debug info (fast to build). `cargo build --release` builds optimised code into `target/release/`.

## From source to running program

```
main.rs                      your text
  ↓  rustc: parse, type-check, borrow-check   ← Rust's safety rules are enforced here
  ↓  rustc → LLVM IR          portable, near-assembly intermediate form
  ↓  LLVM → machine code      arm64 instructions, in pieces
deps/*.rcgu.o                object files
  ↓  Apple's linker           joins the pieces and the standard library
debug/playground             the program
```

## `main`, as the CPU sees it

```rust
fn main() {
    println!("Hello, world!");
}
```

disassembled with `otool -tV target/debug/playground -p <mangled name of main>`:

```
stp  x29, x30, [sp, #-0x10]!   save where we came from
mov  x29, sp
adrp x0, 53                    x0 = address of "Hello, world!\n"
add  x0, x0, #0x9b0
mov  w8, #0xe                  0xe = 14 = length of "Hello, world!\n", known at compile time
mov  x1, x8
bl   …core::fmt::Arguments::from_str
bl   …std::io::stdio::_print   the standard library does the printing
ldp  x29, x30, [sp], #0x10     restore
ret
```

- `x0`, `x1`, `x29`, `x30` are registers. `bl` means "branch with link", i.e. call a function.
- `__RNvCs…10playground4main` is a **mangled** name, the compiler's unique label for the
  function. Find it with `nm target/debug/playground | grep main`; it changes with the code.

## Follow-ups

- Emit the intermediate stages yourself: `cargo rustc -- --emit=llvm-ir,asm` (output in `target/debug/deps/`).
- Compare the `dev` and `--release` disassembly of the same function.
- See also [setup.md](setup.md), which has the `aarch64-apple-darwin` questions.
