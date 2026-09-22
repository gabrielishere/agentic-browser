# Rust setup

**Installed:** 2026-09-22 · Rust **1.98.1** (`48a229cea 2026-09-01`), stable

## What happened

- Installed with `rustup`, the official installer, using the command from
  [rust-lang.org/tools/install](https://rust-lang.org/tools/install/).
- **Installed just for this user, not the whole machine.** No `sudo`. Other macOS users can't
  see it, which a separate agent user (S1) would need as its own install.
  - `~/.rustup`: toolchains (the compiler versions)
  - `~/.cargo/bin`: the commands (`cargo`, `rustc`, `rustup`, …)
- Added `~/.cargo/bin` to `PATH` by editing `~/.profile`, `~/.zshenv` and `~/.tcshrc`.
- Components: `rustc` (compiler), `cargo` (build and packages), `rust-std` (standard
  library), `clippy` (linter), `rustfmt` (formatter), `rust-docs` (offline docs:
  `rustup doc --book`).
- Target: `aarch64-apple-darwin`.
- Uninstall everything with `rustup self uninstall`.

## To dig into later: `aarch64-apple-darwin`

A "target triple" names the platform the compiler builds for:
**architecture – vendor – operating system**.

| Part | Means |
|---|---|
| `aarch64` | 64-bit ARM, the instruction set of Apple Silicon (the A18 Pro here) |
| `apple` | vendor |
| `darwin` | the kernel under macOS |

Questions to follow up:

- How does `rustc` turn Rust into ARM machine code? It goes through LLVM, the same backend
  Apple's own Clang uses.
- What does Rust's support tier for this target guarantee? See
  [Platform support](https://doc.rust-lang.org/rustc/platform-support.html).
- Cross-compiling for Intel Macs (`x86_64-apple-darwin`): `rustup target list`,
  `rustup target add`.
- The linker comes from Apple's command-line tools. What does it do after `rustc`?
- See every target the compiler knows: `rustc --print target-list`.
