---
name: rust-guidelines
description: Microsoft's Pragmatic Rust Guidelines, packaged for this workspace. Use whenever writing, reviewing, refactoring, or designing Rust code here (Dioxus 0.7 fullstack workspace) — covers API design, error handling, async, performance, docs, correctness, and AI-specific rules. Load before touching any .rs file.
---

# Pragmatic Rust Guidelines (Microsoft)

Vendored from <https://microsoft.github.io/rust-guidelines/> (MIT, Microsoft Corporation).
Upstream "agents" bundle: <https://microsoft.github.io/rust-guidelines/agents/all.txt>.

## How to use this skill

1. Skim the condensed checklist below — it is the subset that applies to this repo.
2. For the exact wording, rationale, and code examples, read the relevant chapter:
   `references/chapters/<chapter>.md`.
3. To find the rule for a topic, grep `references/index.md` (all 90 rules, one line each).
4. `references/all.txt` is the complete upstream text in one file.

Chapter files in `references/chapters/`:

| File | Covers |
| --- | --- |
| `ai-guidelines.md` | Agent-specific rules (tautological tests, single item path, Rust-shaped code) |
| `universal-guidelines.md` | Naming, lints, logging, Debug/Display, crate splitting |
| `libraries-interoperability-guidelines.md` | Send, foreign re-exports, `impl AsRef`, `impl RangeBounds` |
| `libraries-ux-guidelines.md` | Errors, builders, async fn, generics vs dyn, parameter order |
| `libraries-resilience-guidelines.md` | Strong types, statics, mockable I/O, tests, glob re-exports |
| `libraries-building-guidelines.md` | Feature flags, out-of-the-box usability, `-sys` crates |
| `macros-guidelines.md` | Macro hygiene, proc-macro crate layout |
| `application-guidelines.md` | App-level errors (Anyhow), allocator, target-cpu |
| `ffi-guidelines.md` | FFI naming, translation-only layers |
| `correctness-guidelines.md` | Panics vs errors, unsafe, soundness |
| `performance-guidelines.md` | Hot path, allocations, async stack size, throughput |
| `project-guidelines.md` | Workspace layout, editions, MSRV |
| `documentation.md` | Doc sections, summary sentences, `#[doc(inline)]` |

## Precedence in this repo

Project rules win over the guidelines where they conflict. Known deviations:

- **Layout** — crates live in `packages/` (`packages/api`, `packages/web`, `packages/desktop`,
  `packages/mobile`, `packages/ui`), not `crates/`. The rule that matters is *no crate nested
  inside another crate's `src/`* (M-CRATES-FLAT-FOLDER); the folder name is a project choice.
- **Workspace deps** — still use `[workspace.dependencies]` + `dep.workspace = true`
  (M-CRATES-IN-WORKSPACE). The root `Cargo.toml` already declares `dioxus`, `ui`, `api`.
- **Dioxus 0.7** — `cx`, `Scope`, and `use_state` no longer exist. Use `use_signal`,
  `use_memo`, `use_resource`, `use_context_provider`, `Signal`, `ReadOnlySignal`.
  This overrides any guideline example that assumes the 0.5/0.6 API.
- **Fullstack** — `packages/api` holds server functions. On the client they become HTTP stubs,
  so API signatures there are the real public contract; follow the library rules (M-DONT-LEAK-TYPES,
  M-ERRORS-CANONICAL-STRUCTS) for anything in `packages/api`.
- **Applications vs libraries** — `packages/web`, `packages/desktop`, `packages/mobile` are
  applications: app-level error handling with `anyhow`/derivatives is allowed (M-APP-ERROR);
  `packages/api` and `packages/ui` are libraries: use canonical error structs.

## Checklist — apply while writing

### Naming and API shape
- Short names, max two words; no crate/module prefix baked in (`foo::Id`, not `foo::FooId`) — M-SHORT-NAMES.
- No weasel words: `Service`, `Manager`, `Factory` are almost always wrong — M-WEASEL-WORDS.
- Prefer plain functions over associated functions that do not belong to the receiver — M-REGULAR-FN.
- Prefer concrete types over generics, generics over `dyn` traits — M-DI-HIERARCHY.
- Avoid smart pointers and wrappers in public APIs — M-AVOID-WRAPPERS.
- Abstractions must not visibly nest (no `foo.bar().baz()` chains) — M-SIMPLE-ABSTRACTIONS.
- Public items reachable by exactly one path; never re-export the same item twice — M-SINGLE-ITEM-PATH.
- Re-export with explicit names, never `pub use foo::*` — M-NO-GLOB-REEXPORTS.
- Foreign types re-exported from their original crate, not duplicated — M-FOREIGN-REEXPORTS.
- Accept `impl AsRef<str>`, `impl RangeBounds<T>`, `impl Read`/`impl Write` where feasible — M-IMPL-ASREF, M-IMPL-RANGEBOUNDS, M-IMPL-IO.
- Public types implement `Debug`; user-facing types implement `Display` — M-PUBLIC-DEBUG, M-PUBLIC-DISPLAY.
- Types are `Send` where possible; `async fn` over returning a `Future` — M-TYPES-SEND, M-ASYNC-FN.
- Consistent parameter ordering across the crate (self, then source, then dest) — M-PARAMETER-CONSISTENCY.
- Services are `Clone` — M-SERVICES-CLONE.
- Essential functionality stays inherent, not only on an extension trait — M-ESSENTIAL-FN-INHERENT.
- No preludes — M-NO-PRELUDE.

### Errors and correctness
- Library errors are canonical structs with `Display` + `std::error::Error`, holding a `Backtrace` — M-ERRORS-CANONICAL-STRUCTS.
- Error conversion via `impl From`, not `map_err` — M-FROM-ERROR.
- Detected programming bugs panic; recoverable conditions return errors — M-PANIC-ON-BUG, M-PANIC-IS-STOP.
- Panic messages state what went wrong and the value — M-PANIC-MESSAGE.
- `unsafe` needs a `// SAFETY:` comment and should be avoided; all code must be sound — M-UNSAFE, M-UNSOUND.
- Never unwrap/expect in library code paths; no `unwrap()` on user input — M-PANIC-IS-STOP.

### Types and invariants
- Use the right type family (`u32` counts, `Duration` times, `Path` paths) — M-STRONG-TYPES.
- Newtypes guard their invariants and expose typed accessors — M-STRONG-TYPES-GUARD.
- Complex construction goes through a builder; builders validate in `.build()` — M-INIT-BUILDER, M-BUILD-RESULT.
- Cascaded initialization: `Foo::new(a).with_b(b).build()` — M-INIT-CASCADED.

### State, statics, and resilience
- Avoid `static mut` and global mutable state; prefer owned state or `OnceLock`/`LazyLock` — M-AVOID-STATICS.
- I/O and system calls must be mockable (inject via traits/`impl` params) — M-MOCKABLE-SYSCALLS.
- Test-only utilities are behind a feature flag — M-TEST-UTIL.
- Integration tests live in `tests/` — M-INTEGRATION-TESTS.
- Production code uses `tracing`/telemetry, never `println!` — M-LOG-NOT-PRINT.

### Docs
- First doc sentence is one line, under ~15 words — M-FIRST-DOC-SENTENCE.
- Every public module has `//!` docs covering what it holds and when to use it — M-MODULE-DOCS.
- Public items use canonical doc sections: `# Examples`, `# Errors`, `# Panics`, `# Safety` — M-CANONICAL-DOCS.
- No "Parameters" tables; explain parameters in prose — M-CANONICAL-DOCS.
- `pub use` of own-crate items gets `#[doc(inline)]` — M-DOC-INLINE.
- No meta design narratives or "rules I followed" tables in user-facing docs — M-NO-META-DESIGN-DOCUMENTATION.

### Performance
- Profile and optimize the hot path early; do not micro-optimize cold code — M-HOTPATH.
- Optimize for throughput; avoid empty polling cycles — M-THROUGHPUT.
- Long-running tasks have yield points — M-YIELD-POINTS.
- Reuse allocations; shrink collections after building; pre-size with `with_capacity` — M-MEM-REUSE, M-SHRINK-TO-FIT, M-INITIAL-CAPACITY.
- Boxed slices/strings (`Box<[T]>`, `Box<str>`) for immutable owned sequences — M-BOX-DST.
- Fast hasher for internal maps — M-FAST-HASHER.
- Avoid needless indirection in nested type hierarchies — M-AVOID-INDIRECTION.
- Keep hot `async fn` bodies small to limit stack size — M-ASYNC-STACK-SIZE.
- Telemetry must not tank performance — M-LOG-OVERHEAD.

### Project hygiene
- Lint overrides use `#[expect(..., reason = "...")]`, never bare `#[allow]` — M-LINT-OVERRIDE-EXPECT.
- Enable the recommended compiler and clippy lint sets — M-STATIC-VERIFICATION.
- Magic values become named constants with a comment explaining the choice — M-DOCUMENTED-MAGIC.
- Structured logging with message templates, named events, redacted sensitive data — M-LOG-STRUCTURED.
- New crates target the latest edition (2024); MSRV trails the current release — M-LATEST-EDITION, M-MSRV.
- If a module can stand alone, split it into its own crate — M-SMALLER-CRATES.
- Features are additive; a crate must build with default features — M-FEATURES-ADDITIVE, M-OOBE.

### Tests
- Tests assert observable behavior, not a restatement of the implementation — M-TAUTOLOGICAL-TESTS.
- Cover error paths, not just the happy path.
- No tautological assertions such as `assert_eq!(CONSTANTS, CONSTANTS)`.

### AI-specific
- Rust code solves Rust problems; do not port C#/Java/Python patterns 1:1 — M-RUST-SHAPED.
- Prefer idiomatic Rust APIs over clever abstractions; APIs should be testable — M-DESIGN-FOR-AI.
- Dioxus specifics: hooks only in components, no `cx`/`Scope`, signals instead of `use_state` — see `AGENTS.md`.

## Before finishing a Rust change

- [ ] `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test` pass.
- [ ] New public items have `Debug` (+ `Display` where user-facing) and doc comments with a summary sentence.
- [ ] Errors follow the canonical struct pattern in `packages/api`; no `unwrap`/`expect` on fallible input.
- [ ] No new `static mut`, no glob re-exports, no duplicate export paths.
- [ ] Tests cover behavior, not implementation shape.
