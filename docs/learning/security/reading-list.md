# Security reading list

**Made:** 2026-09-24 · Pointers only. Full source notes: [`research/security-sources.md`](../../research/security-sources.md)

In order. Each line says what to read it for.

## How browsers defend themselves

1. [The Rule of 2](https://chromium.googlesource.com/chromium/src/+/main/docs/security/rule-of-2.md) — Chromium. Untrusted input, unsafe language, high privilege: pick at most two. *(Read together on 2026-09-24.)*
2. [The Security Architecture of the Chromium Browser](https://seclab.stanford.edu/websec/chromium/chromium-security-architecture.pdf) — Barth, Jackson, Reis et al., 2008. Why a browser splits into a privileged kernel and sandboxed renderers.
3. [Web Browser Engineering, ch. "Keeping Data Private"](https://browser.engineering/security.html) — Panchekha & Harrelson. Cookies, same-origin policy and cross-site attacks, built step by step.
4. [Mac Sandbox V2 design](https://chromium.googlesource.com/chromium/src/+/main/sandbox/mac/seatbelt_sandbox_design.md) and [README](https://chromium.googlesource.com/chromium/src/+/HEAD/sandbox/mac/README.md) — Chromium. What sandboxing means on macOS (D32 step 2).
5. [Ladybird process architecture](https://github.com/LadybirdBrowser/ladybird/blob/master/Documentation/ProcessArchitecture.md) — the closest project to this one; short.
6. [Process Model and Site Isolation](https://chromium.googlesource.com/chromium/src/+/main/docs/process_model_and_site_isolation.md) — Chromium. Needed for the JS phase, not before. Skim.
7. [Retrofitting Fine Grain Isolation in the Firefox Renderer](https://www.usenix.org/conference/usenixsecurity20/presentation/narayan) — USENIX Security 2020 (RLBox). Sandboxing one library instead of a whole process.

## Where attacks actually come from

8. [Memory safety](https://www.chromium.org/Home/chromium-security/memory-safety/) — Chromium. The ~70% figure.
9. [Is Rust Used Safely by Software Developers?](https://arxiv.org/abs/2007.00752) — Evans et al., ICSE 2020. "Safe Rust" is your code, not your dependency tree.
10. [SSRF Prevention Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Server_Side_Request_Forgery_Prevention_Cheat_Sheet.html) — OWASP. The attack Rust doesn't prevent (T2).

## The build pipeline — live now

11. [Agents Rule of Two](https://ai.meta.com/blog/practical-ai-agent-security/) — Meta, 2025. The Chromium rule applied to agents.
12. [The lethal trifecta](https://simonwillison.net/2025/Jun/16/the-lethal-trifecta/) — Simon Willison, 2025. Short.
13. [Nx s1ngularity postmortem](https://nx.dev/blog/s1ngularity-postmortem) — a real attack that used AI coding CLIs to steal secrets (T12).
14. [Malicious crate rustdecimal](https://blog.rust-lang.org/2022/05/10/malicious-crate-rustdecimal/) — Rust blog. Supply chain on crates.io (T9).
15. [Sandboxing build.rs and proc-macros](https://internals.rust-lang.org/t/sandbox-build-rs-and-proc-macros/16345) — Rust internals. Why Cargo isn't a security boundary.
16. [cargo-vet](https://mozilla.github.io/cargo-vet/) — Mozilla. One answer to "who approves a dependency" (S3).
17. Claude Code [Sandboxing](https://code.claude.com/docs/en/sandboxing) and [Permissions](https://code.claude.com/docs/en/permissions). What the tool you build with actually enforces (S8).

## If there's time

- [Apple Platform Security](https://support.apple.com/guide/security/welcome/web): what a second macOS user isolates (S1). The exact section hasn't been found yet.
- [Rust Fuzz Book](https://rust-fuzz.github.io/book/): for later (S5).
- *The Tangled Web* — Zalewski ([No Starch](https://nostarch.com/tangledweb)). A book, not a day.
