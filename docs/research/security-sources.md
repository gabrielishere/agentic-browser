# Security — sources

**Gathered:** 2026-09-22
**Serves:** [`design/security.md`](../design/security.md) and the
[threat model](../security/threat-model.md)

> A reading list with the reason each item is on it. Every entry is linked to a
> primary source where one exists.
>
> **How these were checked.** Found by web search on 2026-09-22 and read via the
> search tool's summaries; only the Ladybird document was fetched directly.
> **None has been read end to end.** Quoted figures are what the summaries
> reported and should be confirmed against the linked page before being relied
> on in a decision.
>
> §0 lists where the sources **disagree with what `design/security.md` or the
> threat model currently says**. That section is the point of the exercise.

---

## 0. Where the sources change the draft

| # | The draft says | The sources say | Affects |
|---|---|---|---|
| 1 | T2 and §3.1: refuse private address ranges with a **denylist** | OWASP: *"do not mitigate SSRF via the use of a deny list or regular expression"*; watch for DNS rebinding and time-of-check/time-of-use races ([A10:2021](https://owasp.org/Top10/2021/A10_2021-Server-Side_Request_Forgery_(SSRF)/)). A browser cannot use an allowlist — the [cheat sheet](https://cheatsheetseries.owasp.org/cheatsheets/Server_Side_Request_Forgery_Prevention_Cheat_Sheet.html)'s "requests to any external address" case applies | T2, §3.1 |
| 2 | §1: memory safety is *"largely present by construction — safe Rust"* | Evans et al.: over half of the most-downloaded crates, and of Servo's dependencies, use `unsafe` directly; unsafety propagates through call chains so most crates cannot be fully checked by the compiler ([ICSE 2020](https://dl.acm.org/doi/10.1145/3377811.3380413)). "Safe Rust" describes our code, not our dependency tree | §1, §3.4, S4 |
| 3 | §2.3: build scripts are a risk to *approve* | Cargo does not treat itself as a security boundary against the code it compiles ([internals thread](https://internals.rust-lang.org/t/sandbox-build-rs-and-proc-macros/16345)); **rust-analyzer builds a dependency as soon as it is added to `Cargo.toml`** — approval after the fact is too late. Sandboxed build scripts are a [Rust project goal](https://rust-lang.github.io/rust-project-goals/2024h2/sandboxed-build-script.html), not shipped as far as found | T9, T12, S3 |
| 4 | §2.1, S8: Claude Code's sandbox is an unknown | It is Seatbelt on macOS; writes limited to the working directory; network only via a proxy; **subagents inherit it**; MCP servers and hooks are **not** constrained; Bash permission rules *"match text, not programs"* and are not a security boundary ([sandboxing](https://code.claude.com/docs/en/sandboxing), [permissions](https://code.claude.com/docs/en/permissions)) | S2, S8 |
| 5 | T12 is hypothetical | It has happened. The Nx compromise (Aug 2025) ran `claude --dangerously-skip-permissions` on victims' machines to hunt for SSH keys, tokens and `.env` files ([Nx postmortem](https://nx.dev/blog/s1ngularity-postmortem), [Snyk](https://snyk.io/blog/weaponizing-ai-coding-agents-for-malware-in-the-nx-malicious-package/)) | T12, S1 |
| 6 | The builder method runs unattended | Meta's **Agents Rule of Two**: an agent should hold at most two of *untrustworthy input*, *access to private data or sensitive systems*, *ability to change state or communicate externally* — with all three it *"should not be permitted to operate autonomously"* ([Meta](https://ai.meta.com/blog/practical-ai-agent-security/)). The implementor, as configured, holds all three | S1, S2, S9, ADR 0008 |

---

## 1. How production browsers defend themselves

| Source | What it is | Why it is here |
|---|---|---|
| [The Rule of 2](https://chromium.googlesource.com/chromium/src/+/master/docs/security/rule-of-2.md) — Chromium | Policy: new code may combine at most two of untrustworthy input, an unsafe language, high privilege | **The single most useful test for this project.** Apply it per component: a Rust parser (untrusted + safe) may run privileged; embedding QuickJS (untrusted + C) may not, unless sandboxed |
| [The Security Architecture of the Chromium Browser](https://seclab.stanford.edu/websec/chromium/chromium-security-architecture.pdf) — Barth, Jackson, Reis, Google Chrome Team, 2008 | The paper that introduced the sandboxed-renderer design | Why a browser kernel is split from a rendering engine; the reasoning behind D30/D31 |
| [Mac Sandbox V2 Design Doc](https://chromium.googlesource.com/chromium/src/+/main/sandbox/mac/seatbelt_sandbox_design.md) and [README](https://chromium.googlesource.com/chromium/src/+/HEAD/sandbox/mac/README.md) — Chromium | How Chrome uses Seatbelt on macOS; a primer on the undocumented profile language | The working reference for D32 step 2 |
| [Process Model and Site Isolation](https://chromium.googlesource.com/chromium/src/+/main/docs/process_model_and_site_isolation.md) — Chromium; [design doc](https://www.chromium.org/developers/design-documents/site-isolation/) | Current process model; one site per process | Needed at the JS phase (§6), not before |
| [Retrofitting Fine Grain Isolation in the Firefox Renderer](https://www.usenix.org/conference/usenixsecurity20/presentation/narayan) — USENIX Security 2020; [RLBox](https://rlbox.dev/); [Mozilla Hacks](https://hacks.mozilla.org/2020/02/securing-firefox-with-webassembly/) | Sandboxing individual libraries (fonts, image decoders) via WebAssembly, inside one process | An alternative to full process separation (D31) — containment per library rather than per process |
| [Ladybird process architecture](https://github.com/LadybirdBrowser/ladybird/blob/master/Documentation/ProcessArchitecture.md) — fetched directly | An independent browser in progress: separate WebContent, RequestServer and ImageDecoder processes | The nearest comparable project. Its own document calls itself *"partly aspirational"*, and its sandboxing is Unix `pledge`/`unveil` with no macOS detail |
| [The Tangled Web](https://nostarch.com/tangledweb) — Michal Zalewski, No Starch, 2011 | A book on why the browser security model is the way it is | Background for §3.1 and §3.2: URL parsing, the same-origin policy, HSTS. Dated on newer features |

## 2. Where real attacks come from

| Source | What it says | Why it is here |
|---|---|---|
| [Memory safety](https://www.chromium.org/Home/chromium-security/memory-safety/) — Chromium | ~70% of serious Chromium security bugs are memory safety; based on 912 high/critical bugs since 2015 | The basis for "a memory-safe language removes most of the risk" — read with §0 row 2 |
| [0-day In the Wild — root cause analyses](https://googleprojectzero.github.io/0days-in-the-wild/rca.html) and [the announcement](https://projectzero.google/2020/07/root-cause-analyses-for-0-day-in-wild.html) — Project Zero | Every publicly known exploited 0-day, with root causes. In 2021, 39 of 58 were memory corruption | **Test for the claim that browser exploits concentrate in JavaScript engines** — not yet checked against the data. Filter the spreadsheet by product and component |
| [QuickJS CVEs](https://www.cvedetails.com/vulnerability-list/vendor_id-24937/product_id-98219/Quickjs-Project-Quickjs.html) and [quickjs-ng](https://www.cvedetails.com/vendor/36727/Quickjs-ng.html) — CVEdetails | Memory-safety CVEs continuing into 2025–2026 (e.g. CVE-2025-12745, CVE-2026-0822) | Evidence for D19's engine choice and for why the JS phase breaks the Rule of 2 |

## 3. Supply chain — crates and build scripts

| Source | What it is | Why it is here |
|---|---|---|
| [RustSec](https://rustsec.org/) · [advisory-db](https://github.com/rustsec/advisory-db) · [cargo-audit](https://github.com/rustsec/rustsec/tree/main/cargo-audit) | Advisory database for crates; `cargo audit` checks `Cargo.lock` against it | Known-vulnerable versions. Also the source for per-crate advisory history (§7) |
| [cargo-deny](https://embarkstudios.github.io/cargo-deny/) — Embark Studios | Checks advisories, bans, licences and **sources** | S3: enforcing where crates may come from, and banning specific crates |
| [cargo-vet](https://mozilla.github.io/cargo-vet/) — Mozilla; [Mozilla's shared audits](https://github.com/mozilla/supply-chain); [LWN coverage](https://lwn.net/Articles/897435/) | Records that each dependency has been audited, and imports others' audits | **The most direct answer to S3** — "who approves a dependency, and where is it recorded" |
| [cargo-geiger](https://github.com/geiger-rs/cargo-geiger) | Counts `unsafe` in a crate and its dependencies | Measures §0 row 2 for our actual dependency tree |
| [RUSTSEC-2022-0042: rustdecimal](https://rustsec.org/advisories/RUSTSEC-2022-0042.html) · [Rust blog](https://blog.rust-lang.org/2022/05/10/malicious-crate-rustdecimal/) · [SentinelOne analysis](https://www.sentinelone.com/labs/cratedepression-rust-supply-chain-attack-infects-cloud-ci-pipelines-with-go-malware/) | A typosquatted crate that downloaded and ran a payload; payload supported macOS | T9 is not theoretical on crates.io |
| [Explore sandboxed build scripts](https://rust-lang.github.io/rust-project-goals/2024h2/sandboxed-build-script.html) — Rust Project Goals 2024h2; [internals discussion](https://internals.rust-lang.org/t/sandbox-build-rs-and-proc-macros/16345) | The state of sandboxing `build.rs` and proc-macros | §0 row 3. Status beyond the 2024h2 goal not established |
| [Is Rust Used Safely by Software Developers?](https://dl.acm.org/doi/10.1145/3377811.3380413) — Evans, Campbell, Soffa, ICSE 2020 ([arXiv](https://arxiv.org/abs/2007.00752)) | Empirical study of `unsafe` across crates.io | §0 row 2. Note: the summaries disagree on one figure (75% vs "more than half") — check the paper |
| [SLSA](https://slsa.dev/) (spec [v1.2](https://slsa.dev/spec/v1.2/about)) | Build-integrity levels and provenance | Mostly for publishers; skim for what applies to one author with no release process |
| [NIST SP 800-218, SSDF v1.1](https://nvlpubs.nist.gov/nistpubs/specialpublications/nist.sp.800-218.pdf); [project page](https://csrc.nist.gov/projects/ssdf) | Secure development practices; four groups (prepare, protect, produce, respond) | A checklist for gaps, not a process to adopt |

## 4. Agents with a shell

| Source | What it is | Why it is here |
|---|---|---|
| [Agents Rule of Two](https://ai.meta.com/blog/practical-ai-agent-security/) — Meta AI, Oct 2025 | The Chromium rule adapted to agents | §0 row 6 — the frame for S1, S2 and S9 |
| [The lethal trifecta](https://simonwillison.net/2025/Jun/16/the-lethal-trifecta/) — Simon Willison, Jun 2025 | Private data + untrusted content + external communication = exfiltration | The precursor to the above; narrower (exfiltration only) |
| [New prompt injection papers](https://simonwillison.net/2025/Nov/2/new-prompt-injection-papers/) — Simon Willison, Nov 2025 | Discusses the Rule of Two and *The Attacker Moves Second* — adaptive attacks defeated twelve published injection defences | Why §2.4 cannot rely on detecting injection |
| [OWASP Top 10 for LLM Applications 2025](https://owasp.org/www-project-top-10-for-large-language-model-applications/assets/PDF/OWASP-Top-10-for-LLMs-v2025.pdf) | LLM01 Prompt Injection, LLM03 Supply Chain, LLM06 Excessive Agency | Standard vocabulary for T3, T4, T12 |
| [Nx s1ngularity postmortem](https://nx.dev/blog/s1ngularity-postmortem) · [StepSecurity](https://www.stepsecurity.io/blog/supply-chain-security-alert-popular-nx-build-system-package-compromised-with-data-stealing-malware) · [Snyk](https://snyk.io/blog/weaponizing-ai-coding-agents-for-malware-in-the-nx-malicious-package/) | Malicious package used installed AI CLIs with permission-bypass flags to inventory secrets | §0 row 5 — T12 in the wild, targeting this exact tool |
| Claude Code: [Sandboxing](https://code.claude.com/docs/en/sandboxing) · [Permissions](https://code.claude.com/docs/en/permissions) · [Security](https://code.claude.com/docs/en/security) · [Sandbox environments](https://code.claude.com/docs/en/sandbox-environments) · [Engineering post](https://www.anthropic.com/engineering/claude-code-sandboxing) | The product's own documentation | §0 row 4 — answers S8 from the source rather than from assumption |

## 5. macOS and the network

| Source | What it is | Why it is here |
|---|---|---|
| [Gatekeeper and runtime protection in macOS](https://support.apple.com/guide/security/gatekeeper-and-runtime-protection-sec5599b66df/web) — Apple Platform Security | How downloaded software is checked before first run | T13 |
| [Apple Platform Security](https://support.apple.com/guide/security/welcome/web) — full guide | The macOS security model | S1: what a second user account does and does not isolate — **not yet looked up specifically** |
| [Resolving Trusted Execution Problems](https://developer.apple.com/forums/thread/706442) — Apple Developer Forums | The quarantine attribute; `curl` does not set it | T13: a browser writing files is in the `curl` position unless it sets the attribute itself |
| [SSRF Prevention Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Server_Side_Request_Forgery_Prevention_Cheat_Sheet.html) — OWASP; [A10:2021](https://owasp.org/Top10/2021/A10_2021-Server-Side_Request_Forgery_(SSRF)/) | Defending against requests to internal addresses | T2 and §0 row 1 |

## 6. Testing defences

| Source | What it is | Why it is here |
|---|---|---|
| [Rust Fuzz Book](https://rust-fuzz.github.io/book/) · [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz) | Fuzzing Rust with libFuzzer; needs nightly; runs on Apple Silicon | §8, S5 |
| [Trail of Bits Testing Handbook — cargo-fuzz](https://appsec.guide/docs/fuzzing/rust/cargo-fuzz/) | A practitioner's guide | §8 |

---

## 7. Evidence to produce locally

Not reading — measurements, once a Cargo workspace exists (Cycle 0). Each tests a
claim above against this project's real dependency tree.

| Measurement | Tool | Tests |
|---|---|---|
| Transitive dependency count of `html5ever`, `winit`, `wgpu`, `rustls`, `swash` | `cargo tree` | The size of T9 |
| `unsafe` in that tree | `cargo-geiger` | §0 row 2 |
| Crates in the tree with `build.rs` or that are proc-macros | `cargo metadata` | §0 row 3 — what runs at compile time |
| Advisory history of each direct dependency | [RustSec](https://rustsec.org/) | How often, and how fast fixed |
| Share of in-the-wild browser 0-days in JS engines | [Project Zero spreadsheet](https://googleprojectzero.github.io/0days-in-the-wild/rca.html) | The claim in §2 of this list |

## Not found or not checked

- A primary account of what a second macOS user account isolates (S1) — the Apple guide is linked but the relevant section was not located.
- Current status of sandboxed build scripts beyond the 2024h2 project goal.
- Whether `rustls` has had an independent audit — not searched.
