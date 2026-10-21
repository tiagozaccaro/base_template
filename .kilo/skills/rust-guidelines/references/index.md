# Rule Index

Every rule in the Microsoft *Pragmatic Rust Guidelines* with its one-line rationale.
Read the chapter file in `chapters/` for the full text, rationale and examples.

Source: <https://microsoft.github.io/rust-guidelines/> (MIT, Microsoft Corporation).

## AI

Source: `chapters/ai-guidelines.md`

| Rule | Summary |
| --- | --- |
| `M-DESIGN-FOR-AI` — Design with AI use in mind | maximum utility from agents working in your codebase. |
| `M-NO-META-DESIGN-DOCUMENTATION` — Avoid meta design documentation | docs focused on what is relevant to users. |
| `M-RUST-SHAPED` — Rust code solves Rust problems | idiomatic code. |
| `M-SINGLE-ITEM-PATH` — Items are only visible through one path | a single, clutter-free path to each type. |
| `M-TAUTOLOGICAL-TESTS` — Tests do not assert ground truth | tests that add value, not noise. |

## Universal

Source: `chapters/universal-guidelines.md`

| Rule | Summary |
| --- | --- |
| `M-DOCUMENTED-MAGIC` — Magic values are documented | maintainability and safe refactoring. |
| `M-LINT-OVERRIDE-EXPECT` — Lint overrides should use `#[expect]` | a current, tidy lint set. |
| `M-LOG-STRUCTURED` — Use structured logging with message templates | low-cost logging with strong filtering. |
| `M-PUBLIC-DEBUG` — Public types are Debug | easy debugging without leaking sensitive data. |
| `M-PUBLIC-DISPLAY` — Public types meant to be read are Display | usability. |
| `M-REGULAR-FN` — Prefer regular over associated functions | readability. |
| `M-SHORT-NAMES` — Names of items are short | idiomatic code. |
| `M-SMALLER-CRATES` — If in doubt, split the crate | fast compile times and good modularity. |
| `M-STATIC-VERIFICATION` — Use static verification | consistency and freedom from common issues. |
| `M-UPSTREAM-GUIDELINES` — Follow the upstream guidelines | a codebase that reflects community lessons and does not surprise users or contributors. |
| `M-WEASEL-WORDS` — Names are free of weasel words | readability. |

## Libraries / Interoperability

Source: `chapters/libraries-interoperability-guidelines.md`

| Rule | Summary |
| --- | --- |
| `M-DONT-LEAK-TYPES` — Don't leak external types | stable APIs and low long-term maintenance cost. |
| `M-ESCAPE-HATCHES` — Native escape hatches | workarounds for unsupported use cases until alternatives exist. |
| `M-FOREIGN-REEXPORTS` — Items come from their original crate | unambiguous type identity. |
| `M-IMPL-ASREF` — Accept `impl AsRef<>` where feasible | flexibility for callers to use their own types. |
| `M-IMPL-IO` — Accept `impl 'IO'` where feasible ('sans IO') | business logic untangled from I/O, with N*M composability. |
| `M-IMPL-RANGEBOUNDS` — Accept `impl RangeBounds<>` where feasible | flexibility and clarity when specifying ranges. |
| `M-TYPES-SEND` — Types are Send | use in Tokio and behind runtime abstractions. |

## Libraries / UX

Source: `chapters/libraries-ux-guidelines.md`

| Rule | Summary |
| --- | --- |
| `M-ASYNC-FN` — Functions are `async` over returning a Future | simpler code and easier-to-understand APIs. |
| `M-AVOID-WRAPPERS` — Avoid smart pointers and wrappers in APIs | low cognitive load and ergonomic APIs. |
| `M-BALANCED-MODULES` — Modules are balanced in size and scope | discoverable functionality and clear API usage. |
| `M-COLLECTION-TRAITS` — Collections implement the appropriate iter traits | composable collections. |
| `M-DI-HIERARCHY` — Prefer types over generics, generics over dyn traits | composable patterns and freedom from design lock-in. |
| `M-ERRORS-CANONICAL-STRUCTS` — Errors are canonical structs | harmonized error types and consistent error handling. |
| `M-ESSENTIAL-FN-INHERENT` — Essential functionality should be inherent | easily discoverable essential functionality. |
| `M-FROM-ERROR` — Canonical error conversion uses `From`, not `map_err` | idiomatic error handling. |
| `M-INIT-BUILDER` — Complex type construction has builders | future-proof type construction in complex scenarios. |
| `M-INIT-CASCADED` — Complex type initialization hierarchies are cascaded | construction free of parameter mix-ups. |
| `M-NO-PRELUDE` — Don't define preludes | a clean namespace and reliable downstream builds. |
| `M-PARAMETER-CONSISTENCY` — Parameter ordering is consistent | low development friction. |
| `M-SERVICES-CLONE` — Services are Clone | composable sharing of common services. |
| `M-SIMPLE-ABSTRACTIONS` — Abstractions don't visibly nest | low cognitive load and a good out-of-the-box UX. |

## Libraries / Resilience

Source: `chapters/libraries-resilience-guidelines.md`

| Rule | Summary |
| --- | --- |
| `M-AVOID-STATICS` — Avoid statics | consistency and correctness across crate versions. |
| `M-BUILD-RESULT` — Builders validate in final `.build()` | clean builder error handling. |
| `M-INTEGRATION-TESTS` — Integration tests live under `tests/` | clean code files. |
| `M-LOG-NOT-PRINT` — Production code uses telemetry, not println | diagnostics available where they are needed. |
| `M-MOCKABLE-SYSCALLS` — I/O and system calls are mockable | testable edge cases that are otherwise hard to evoke. |
| `M-NO-GLOB-REEXPORTS` — Don't glob re-export items | a deliberate public surface. |
| `M-STRONG-TYPES-GUARD` — Newtypes guard their invariants | centralized correctness invariants. |
| `M-STRONG-TYPES` — Use the proper type family | the right data and safety invariants, at the right time. |
| `M-TEST-UTIL` — Test utilities are feature gated | production builds that cannot bypass safety checks. |

## Libraries / Building

Source: `chapters/libraries-building-guidelines.md`

| Rule | Summary |
| --- | --- |
| `M-FEATURES-ADDITIVE` — Features are additive | reliable compilation in large, complex projects. |
| `M-OOBE` — Libraries work out of the box | easy adoption across the Rust ecosystem. |
| `M-SYS-CRATES` — Native `-sys` crates compile without dependencies | libraries that just work on all platforms. |

## Macros

Source: `chapters/macros-guidelines.md`

| Rule | Summary |
| --- | --- |
| `M-EXAMPLE-OVER-PROC` — Prefer 'macros by example' over proc macros | easy macro inspection and fast compilation. |
| `M-MACRO-HELPERS` — Third party items come from hidden `_private` module | predictable compilation. |
| `M-MACRO-LAST-RESORT` — Macros are a last resort | minimal complexity. |
| `M-MACRO-MAIN-CRATE` — Macros assume main crate | simple macro logic. |
| `M-MACRO-VERSION-PIN` — Pin supporting proc macro crates | keep generated code compatible with its library |
| `M-MACROS-DONT-LIE` — Macros don't lie about signatures | clarity for users and LLMs. |
| `M-PROC-IMPL` — Proc macros should have separate impl crate incl. tests | thoroughly testable proc macros. |
| `M-PROC-IMPLIED-ITEMS` — Proc macros don't produce implied or hidden items | clear errors and correct hygiene and visibility. |

## Applications

Source: `chapters/application-guidelines.md`

| Rule | Summary |
| --- | --- |
| `M-APP-ERROR` — Applications may use Anyhow or derivatives | simple application-level error handling. |
| `M-MIMALLOC-APPS` — Use mimalloc for apps | significant performance at no cost. |
| `M-TARGET-CPU` — Applications target highest viable target-cpu | fleet performance. |

## FFI

Source: `chapters/ffi-guidelines.md`

| Rule | Summary |
| --- | --- |
| `M-FFI-NAMING` — FFI crates follow established naming conventions | immediately recognizable crate roles across projects. |
| `M-FFI-TRANSLATES` — Business logic belongs in core crates, FFI only translates | maximal safe code and a clean separation of concerns. |
| `M-ISOLATE-DLL-STATE` — Isolate DLL state between FFI libraries | data integrity and defined behavior across DLL boundaries. |

## Correctness

Source: `chapters/correctness-guidelines.md`

| Rule | Summary |
| --- | --- |
| `M-PANIC-CONTINUATION` — Panic continuation is last resort | state integrity and freedom from subtle bugs. |
| `M-PANIC-IS-STOP` — Panic means 'stop the program' | soundness and predictability. |
| `M-PANIC-MESSAGE` — Custom panics have a helpful message | faster bug diagnosis. |
| `M-PANIC-ON-BUG` — Detected programming bugs are panics, not errors | tractable error handling and runtime consistency. |
| `M-UNSAFE-IMPLIES-UB` — Unsafe implies undefined behavior | semantic consistency without warning fatigue. |
| `M-UNSAFE` — Unsafe needs reason, should be avoided | memory safety and a minimal attack surface. |
| `M-UNSOUND` — All code must be sound | predictable runtime behavior free of bugs and incompatibilities. |

## Performance

Source: `chapters/performance-guidelines.md`

| Rule | Summary |
| --- | --- |
| `M-ASYNC-STACK-SIZE` — Hot `async` functions reduce stack size | small async stack sizes and low memcpy overhead. |
| `M-AVOID-INDIRECTION` — Nested type hierarchies should avoid needless indirection | fast, cache-friendly memory access. |
| `M-BOX-DST` — Use boxed slices and strings for immutable owned sequences | low memory consumption and good cache utilization. |
| `M-FAST-HASHER` — Use a fast hasher where possible | hashing performance. |
| `M-HOTPATH` — Identify, profile, optimize the hot path early | high-performance code. |
| `M-INITIAL-CAPACITY` — Collections are created with sufficient initial capacity | efficient collection creation. |
| `M-LOG-OVERHEAD` — Library telemetry does not tank performance | low-overhead telemetry during diagnosis. |
| `M-MEM-REUSE` — Reuse allocations where possible | low allocation overhead and fast hot paths. |
| `M-SHRINK-TO-FIT` — Shrink collections to fit after building | a minimal memory footprint. |
| `M-THROUGHPUT` — Optimize for throughput, avoid empty cycles | COGS savings at scale. |
| `M-YIELD-POINTS` — Long-running tasks should have yield points | fair CPU time for all tasks. |

## Project

Source: `chapters/project-guidelines.md`

| Rule | Summary |
| --- | --- |
| `M-CARGO-WORKSPACE` — Common settings come from the workspace Cargo.toml | consistent, maintainable project configuration. |
| `M-CRATES-FLAT-FOLDER` — All crates are siblings in one folder | simple project navigation and a standard Rust layout. |
| `M-CRATES-IN-WORKSPACE` — The workspace lists and versions all crates | simple inter-crate dependencies and debugging. |
| `M-LATEST-EDITION` — New crates target latest edition | access to the latest Rust features. |
| `M-MSRV` — MSRV is conservatively updated | modern features with stability for users. |

## Documentation

Source: `chapters/documentation.md`

| Rule | Summary |
| --- | --- |
| `M-CANONICAL-DOCS` — Documentation has canonical sections | established Rust documentation practices. |
| `M-DOC-INLINE` — Mark `pub use` items with `#[doc(inline)]` | re-exported items that fit in with their siblings. |
| `M-FIRST-DOC-SENTENCE` — First sentence is one line; approx. 15 words | easily skimmable API docs. |
| `M-MODULE-DOCS` — Has comprehensive module documentation | easy API docs navigation. |

Total rules: 90
