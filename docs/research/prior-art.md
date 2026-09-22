# Prior art — open-source browsers and engines

**Gathered:** 2026-09-22
**Serves:** learning by review ([ADR 0008](../adr/0008-agents-build-author-specifies-and-reviews.md) D44), and reference for every stage of [ADR 0007](../adr/0007-build-order-and-optional-features.md)

> Found by web search on 2026-09-22 and read via the search tool's summaries;
> the Ladybird document was fetched directly. Licences and status should be
> confirmed on each project's own page before they matter to a decision.
>
> Security-specific sources — Chromium's design docs, sandboxing, supply chain —
> are in [`security-sources.md`](security-sources.md), not repeated here.

---

## Read first — learning resources

| Project | What it is | Why it matters here |
|---|---|---|
| **robinson** — [code](https://github.com/mbrubeck/robinson) · [blog series, part 1 of 7](https://limpet.net/mbrubeck/2014/08/08/toy-layout-engine-1.html) · [talk slides](https://limpet.net/mbrubeck/robinson-talk/) | A toy engine in Rust by Matt Brubeck, written at Mozilla purely to teach. No external crates | Almost exactly ADR 0001 rung 1: HTML subset → DOM, CSS, style, block layout, paint. The companion text for reviewing stage 1 cycles |
| **Web Browser Engineering** — [free online](https://browser.engineering/) · [Oxford University Press](https://global.oup.com/academic/product/web-browser-engineering-9780198913863) | Pavel Panchekha and Chris Harrelson (who leads Google's Blink rendering team). Builds a complete browser in about 2,000 lines of Python | The whole arc — networking, layout, styles, JavaScript, security — explained. Open access under CC BY-NC-ND 4.0 |

## Rust engines

| Project | What it is | Why it matters here |
|---|---|---|
| **Blitz** — [code](https://github.com/DioxusLabs/blitz) · [Web Engines Hackfest 2024 slides](https://webengineshackfest.org/2024/slides/blitz_a_truly_modular_hackable_web_renderer_by_nico_burns.pdf) | Dioxus Labs. A no-JavaScript HTML/CSS renderer: `html5ever`, Stylo (styles), Taffy (layout), Parley (text) | The closest cousin to stage 1. It imports style and layout; D15 says we build them. Its `blitz-net` fetches `file://` and `data:` URLs — the opposite of T2's intent |
| **Servo** — [code](https://github.com/servo/servo) · [site](https://servo.org/) · [0.1.0 on crates.io](https://servo.org/blog/2026/04/13/servo-0.1.0-release/) · [Servo Book: LTS](https://book.servo.org/embedding/lts-release.html) | The Rust engine Mozilla started, now under Linux Foundation Europe. Embeddable as a crate since April 2026 | The reference for anything we build. `html5ever` and Stylo come from it |

## Independent engines

| Project | What it is | Why it matters here |
|---|---|---|
| **NetSurf** — [site](https://www.netsurf-browser.org/) · [source](https://www.netsurf-browser.org/downloads/source/) · [libdom](https://source.netsurf-browser.org/libdom.git/tree/src/core/) | A small browser in C with its own layout engine written from scratch; mostly HTML 4 and CSS 2.1 | Roughly our stage 1 scope, finished. Split into readable libraries (libdom, libcss, and others). **GPL 2 — see below** |
| **Ladybird** — [code](https://github.com/LadybirdBrowser/ladybird) · [process architecture](https://github.com/LadybirdBrowser/ladybird/blob/master/Documentation/ProcessArchitecture.md) | An independent browser in progress, in C++ | Separate processes for pages, networking and image decoding. Its architecture document calls itself "partly aspirational" |

## The big three — for their design documents

| Project | Code | Licence |
|---|---|---|
| **Firefox / Gecko** | [github.com/mozilla-firefox/firefox](https://github.com/mozilla-firefox/firefox) · [Searchfox](https://searchfox.org/) | [MPL 2.0](https://www.mozilla.org/en-US/MPL/2.0/) |
| **Chromium** | [chromium.googlesource.com](https://chromium.googlesource.com/chromium/src/) | BSD |
| **WebKit** | [webkit.org](https://webkit.org/) | LGPL / BSD |

---

## Licences — read, don't copy

This project is `MIT OR Apache-2.0` ([ADR 0004](../adr/0004-preserve-optionality-toward-product.md) D25).
Reading any of these is fine. Copying from them is not the same thing, and it
matters more because agents write the code (ADR 0008): an implementor that
reproduces a reference project's code imports that project's licence.

| Licence | Projects | Copying code in |
|---|---|---|
| MIT / Apache / BSD | robinson, Blitz, Chromium, Ladybird | Allowed, with attribution |
| MPL 2.0 | Servo, Firefox | Copied files stay under MPL — mixed licensing |
| **GPL 2** | **NetSurf** | **Would force the project to GPL.** Read only |

**Proposed spec constraint** (not yet adopted): *Reference projects may be read;
code may not be copied from them. Any adapted code names its source and
licence in a comment, and GPL sources are never adapted.*
