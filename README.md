# Paul Insider

**Rust engineering for thoughtful digital products.**

Paul Insider designs and develops websites, property applications, SaaS products
and focused AI integrations, with hosting and maintenance defined for each project.

[Explore Paul Insider](https://paulinsider.com/en/) ·
[Discuss a project](https://paulinsider.com/en/#project)

## Selected engineering work

This repository presents one small, dependency-free Rust library with two
independent examples. It makes selected engineering decisions readable and testable.

| Example | What it demonstrates |
| --- | --- |
| [Multilingual entry policy](src/language.rs) | Explicit EN/FR/ES defaults, a reliable English fallback and weighted language preferences for Canada |
| [Media byte ranges](src/media_range.rs) | Validated closed, open-ended and suffix ranges, bounded integer arithmetic and precise media offsets |

These standalone examples cover concerns relevant to the Paul Insider website.
They are presented as public samples; the commercial website and client applications
are maintained separately.

## Inspect, run, test

Requirements: an installed Rust toolchain with Cargo. The crate has no third-party
dependencies and does not make network requests.

```bash
cargo test --locked --offline
cargo run --locked --offline --example web_policies
```

The example prints a localized home path and the bounds of a small video request.
It does not start a server, collect visitor information or submit a form.

## Engineering scope

- Pure functions and explicit result types.
- No unsafe Rust and no third-party dependencies.
- Tests for malformed input, boundary values, overflow limits and actual byte slices.
- Country data must come from a trusted server-side source.
- Explicit language URLs must retain the visitor's choice.
- HTTP authentication, cache policy, conditional requests and streaming belong to the application adapter.

The range example handles one byte range with `u64` offsets. It is intentionally
limited in scope and is not a complete implementation of every HTTP range feature.

## Work with Paul Insider

From a defined business need to a considered interface, an agreed development scope
and delivery: websites, SaaS, applications and integrations built for the project.

[Start the conversation](https://paulinsider.com/en/#project)

## Rights

Published for inspection. Rights and licensing are stated in [LICENSE](LICENSE).
