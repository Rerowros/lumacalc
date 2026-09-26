## 1. Product and Decision Gates

- [ ] 1.1 Record product name placeholder, owner, target distribution, provisional Slint license path, and attribution obligations in `docs/product-decisions.md` (PRIV-007). Gate: human approval is recorded before any public binary is produced.
- [ ] 1.2 Define the exact Windows reference device, OS builds, power settings, display scales, and 20-run measurement method in `PERF_REFERENCE.md` (PERF-001 through PERF-004). Gate: a reviewer can repeat every command from a clean release build.
- [ ] 1.3 Freeze MVP versus post-MVP scope and map every capability/requirement ID to a milestone in a machine-readable traceability file (MODE-007, GRAPH-006). Gate: the traceability validator reports no unmapped normative requirement.
- [ ] 1.4 Create Architecture Decision Records for GUI toolkit, renderer gate, numeric backend gate, storage, concurrency, and packaging (D1-D13). Gate: each ADR lists status, alternatives, decision evidence, and rollback trigger.
- [ ] 1.5 Define stable JSON schemas for expression fixtures, error fixtures, unit fixtures, UI command registry, and benchmark output (EXPR-004, EXPR-007, REL-003). Gate: schemas validate one positive and one intentionally invalid sample.
- [ ] 1.6 Document dependency admission policy covering maintenance, license, unsafe code, binary cost, features, and replacement boundary (PRIV-007, PERF-002). Gate: every planned third-party crate has an owner and review state.

## 2. Rust Workspace and Verification Foundation

- [ ] 2.1 Create the Cargo workspace and directories from Design D3, pin stable Rust in `rust-toolchain.toml`, use Rust 2024 edition, and commit a lockfile (EXPR-007, REL-003). Gate: `cargo metadata --no-deps` succeeds and dependency direction matches the design.
- [ ] 2.2 Add workspace lint profiles that deny warnings in CI, forbid unsafe code in domain crates, and flag forbidden dependencies from domain to UI/platform/storage (REL-003). Gate: an intentional forbidden dependency fails the architecture test.
- [ ] 2.3 Create `calc-test-support` fixture loaders, deterministic clock/ID providers, and JSON Schema validation (EXPR-007). Gate: `cargo test -p calc-test-support` covers valid, invalid, and version-mismatch fixtures.
- [ ] 2.4 Create an `xtask` crate for `verify`, fixture validation, localization completeness, traceability, benchmark orchestration, and packaging checks (REL-003). Gate: `cargo run -p xtask -- verify-fast` runs from a clean checkout.
- [ ] 2.5 Configure formatting, Clippy, unit/integration tests, release build, OpenSpec validation, dependency audit, and license/SBOM checks in Windows CI (REL-003, PRIV-007). Gate: the pipeline produces a machine-readable gate report even on failure.
- [ ] 2.6 Configure scheduled fuzz, performance, accessibility-smoke, and packaging jobs separately from fast pull-request gates (REL-001, PERF-001 through PERF-004). Gate: each job has a manual trigger and artifact retention policy.
- [ ] 2.7 Add repository guidance for AI agents: requirement IDs, allowed module boundaries, test-first fixtures, no silent spec/budget changes, and review requirements for unsafe/dependencies/migrations (D15). Gate: `xtask traceability` detects a sample unreferenced implementation task.

## 3. Native UI and Renderer Technology Spike

- [ ] 3.1 Create a minimal `calculator-desktop` Slint 1.17.x application using `winit` and a production-like window lifecycle (UI-001, PERF-005). Gate: release x64 starts focused and exits without a residual process.
- [ ] 3.2 Implement spike-only dark Fluent semantic design tokens and the compact/regular/expanded layout states with no product math (UI-002, UI-007). Gate: visual snapshots cover dark, light, high contrast, all three widths, and the hierarchy/density checklist from D9.
- [ ] 3.3 Build the representative spike screen: restrained opaque dark surface, collapsible navigation, dominant expression/result area, keypad or converter work area, inline error row, flat conversion/suggestion rows, compact trailing commands, history list, focus states, and Russian sample text (UI-001 through UI-007, UI-011, A11Y-006). Gate: component tests exercise every visible state without blur, traffic-light chrome, neon/glow, or redundant cards/capsules.
- [ ] 3.4 Add keyboard paths and semantic metadata to every spike control, including Narrator-friendly result/error announcements (A11Y-001, A11Y-002). Gate: the scripted keyboard journey completes with no pointer and the accessibility-tree capture has no unnamed control.
- [ ] 3.5 Produce an isolated `winit-software` release profile containing only required renderer features (PERF-002, PERF-003). Gate: dependency tree, executable size, startup, memory, idle, input, resize, DPI, text-quality, and graphics-compatibility evidence is captured.
- [ ] 3.6 Produce the equivalent `winit-femtovg` release profile without changing screen behavior (PERF-002, PERF-003). Gate: the identical evidence bundle and fixture version is captured and compared to software.
- [ ] 3.7 Exercise `winit-femtovg-wgpu` only as a compatibility/graphing experiment and list unsupported visual/accessibility behavior for every profile (UI-007, A11Y-002). Gate: each renderer is explicitly accepted or rejected with evidence; non-selected renderers are not silently included.
- [ ] 3.8 Run the renderer matrix at 100/150/200% DPI, English/Russian, light/dark/high contrast, reduced motion, Narrator, resize stress, and a VM/remote session where available (A11Y-001 through A11Y-007). Gate: all hard failures are classified and reproducible.
- [ ] 3.9 Run the controlled performance matrix for at least 20 release starts per candidate and save raw JSON/traces (PERF-001 through PERF-004). Gate: `xtask bench-report` calculates p50/p95 and refuses missing runs.
- [ ] 3.10 Approve one production renderer or activate the Qt Quick/QML fallback ADR before continuing (D2). Gate: selected profile passes every hard accessibility/resource requirement and non-selected renderer features are removed from the normal build.

## 4. Numeric and Grammar Spikes

- [ ] 4.1 Write grammar v1 in EBNF with precedence/associativity tables for literals, calls, postfix operators, assignments, units, conversions, and programmer operators (EXPR-001). Gate: every grammar production has positive and negative JSON fixtures.
- [ ] 4.2 Specify `%`, repeated operation, unary minus versus exponentiation, implicit multiplication, `Ans`, angle annotations, assignment, and locale separator semantics (EXPR-001, EXPR-002). Gate: ambiguity review produces one accepted result or error fixture per case.
- [ ] 4.3 Define the closed numeric value model, promotion matrix, exactness propagation, 128-bit default precision, rounding mode, exponent range, and safety limits (EXPR-002, EXPR-003, EXPR-005). Gate: the matrix has no unspecified type/operator pair in MVP scope.
- [ ] 4.4 Spike `fend-core` for required scientific functions, units, variables, locale normalization, interruption, formatting, and binary cost (EXPR-003, MODE-001, UNIT-001, PERF-002). Gate: golden fixtures and resource measurements are attached to the engine ADR.
- [ ] 4.5 Compare `fend-core` results against independent curated exact/high-precision fixtures without shipping an oracle dependency (MODE-001, REL-001). Gate: known divergences are reviewed, not hidden by broad epsilon checks.
- [ ] 4.6 Approve or replace the engine behind `CalculatorEngine` before UI math expands (D4, D5). Gate: exactness, scientific/unit coverage, limits, license, maintenance, and package-size criteria all have explicit pass/fail results.
- [ ] 4.7 Define stable error-code catalog and message arguments for parse, symbol, zero division, domain, dimension, overflow, cycle, storage, and limit errors (EXPR-004). Gate: English/Russian placeholder schemas match for every code.
- [ ] 4.8 Freeze fixture schema v1 and generate the first golden corpus for Standard, Scientific, Programmer, units, locale input, and errors (EXPR-007). Gate: independent review confirms fixtures encode expected behavior rather than current implementation output.

## 5. Headless Expression Engine

- [ ] 5.1 Implement bounded locale/Russian alias normalization and the `CalculatorEngine` adapter around `fend-core`, with no host-language execution or network provider (EXPR-001, EXPR-004, UI-004, PRIV-001). Gate: unit/golden/property tests and fuzz smoke pass.
- [ ] 5.2 Implement stable error/result mapping, input/output limits, interruption, canonical history strings, and recovery hints around the engine (EXPR-004, EXPR-005). Gate: all positive/negative fixtures pass and malformed input never panics.
- [ ] 5.3 Implement explicit application numeric context and separate fixed-width `Programmer` values without converting retained engine values through primitive `f64` (EXPR-002, EXPR-003). Gate: context and programmer promotion tests pass.
- [ ] 5.4 Implement Standard arithmetic, parentheses, constants, powers, roots, factorial limits, `Ans`, optional leading-equals input, contextual calculator percent semantics, and repeated binary operation (EXPR-001, EXPR-003, EXPR-008, EXPR-009). Gate: golden fixtures include `=5+5`, `10%`, `100+50%`, `200-10%`, `200*10%`, and `200/10%`, plus property tests.
- [ ] 5.5 Implement structured symbol tables, immutable variable snapshots, assignment, overwrite handling, and cycle detection (STATE-003, STATE-004). Gate: direct/indirect cycle and replay fixtures pass.
- [ ] 5.6 Implement formatter modes, significant digits, grouping, exact/approx marker, locale display, and invariant full-precision copy (EXPR-003, EXPR-006). Gate: parse-format and locale round-trip properties pass.
- [ ] 5.7 Add cooperative generation-token cancellation and limits for source, tokens, AST, precision, exponent, factorial, elapsed work, and output digits (EXPR-005). Gate: adversarial fixtures terminate within configured time/memory bounds.
- [ ] 5.8 Build a headless CLI/fixture harness with no Slint, SQLite, or Windows dependency (EXPR-007). Gate: `cargo run -p calc-cli -- --fixture <id>` matches the library result byte-for-byte.
- [ ] 5.9 Add parser/evaluator fuzz targets and persist the initial regression corpus (REL-001). Gate: scheduled bounded fuzz run finishes with zero crash/hang and replays saved seeds in normal CI.
- [ ] 5.10 Benchmark parse/evaluate/format hot paths and remove allocations or clones that violate typical 5 ms p95 (PERF-004). Gate: benchmark JSON is stored with compiler and fixture versions.

## 6. Native MVP Vertical Slice and Workspace UX

- [ ] 6.1 Replace spike stubs with typed `AppCommand`, reducer/service effects, immutable `AppSnapshot`, and generation IDs (MODE-006). Gate: reducer tests cover edit, evaluate, stale result, error, and mode transitions without creating a window.
- [ ] 6.2 Bind the Slint expression editor to headless parse/evaluate/format services without arithmetic in `.slint` files (UI-001, MODE-006). Gate: architecture test rejects domain imports in UI and typed/button paths share fixtures.
- [ ] 6.3 Implement committed evaluation, live bounded preview, exact/approx result, inline source-span errors, and focus preservation (UI-001, EXPR-004, EXPR-005). Gate: UI integration tests cover success, error correction, and superseded preview.
- [ ] 6.4 Implement adaptive keypad, hide/show behavior, compact/regular/expanded layouts, and narrow drawer behavior (UI-002). Gate: layout snapshots and 100/150/200% resize tests show no clipped primary action.
- [ ] 6.5 Implement the centralized command registry and required shortcuts, tooltips, command palette, and shortcut-conflict validator (UI-003, UI-006). Gate: the keyboard-only primary journey and registry completeness test pass.
- [ ] 6.6 Implement safe paste, selection, standard undo/redo, clear, history navigation, visible copy, and invariant full-precision copy (UI-004, EXPR-006, PRIV-003). Gate: hostile-paste and clipboard-intent integration tests pass.
- [ ] 6.7 Implement visible mode/angle/base/precision context controls and ensure context changes never mutate committed history models (UI-005, EXPR-002). Gate: context snapshot fixtures pass across DEG/RAD and display-mode changes.
- [ ] 6.8 Finish the shared dark Fluent token system, consistent navigation/result/key states, focus, reduced motion, target density, opaque software-rendered surface, and zero periodic repaint at idle (UI-007, A11Y-004, A11Y-005, PERF-003). Gate: Windows Calculator navigation and PowerToys Run overlay reference comparison, compact/expanded screenshot pair, token-consistency review, theme matrix, and 30-second idle trace pass.
- [ ] 6.9 Add clean lifecycle and worker shutdown handling (PERF-005). Gate: automated close test observes no residual process, file handle, or delayed write.
- [ ] 6.10 Implement the Windows `RegisterHotKey`/`WM_HOTKEY` adapter for `Win+Alt+Space` with `MOD_NOREPEAT`, conflict reporting, retry, and guaranteed unregister on Quit (UI-009, UI-010, PERF-005). Gate: adapter tests plus a live Windows hotkey smoke test pass.
- [ ] 6.11 Add compact 680px-wide content-height PowerToys Run-style overlay with a leading-edge focused expression editor, flat results, and compact Expand, Hide, and Quit commands (UI-002, UI-009). Gate: error, suggestion, and four-conversion screenshots contain no reserved blank region; the overlay opens from hidden state, accepts keyboard input immediately, and reflows at 100/150/200% DPI.
- [ ] 6.14 Add a dense full-window dark calculator shell with minimal draggable chrome, collapsible calculator/converter/settings navigation, one active-mode label, compact result area, and mode-specific work area (UI-007, UI-011, UNIT-009). Gate: pointer and keyboard navigation switch every implemented mode in one window, no engine metadata or duplicate title is visible, and visual QA passes at 100/150/200% DPI.
- [ ] 6.15 Add `--background` hidden launch plus an explicit reversible launch-at-login Settings toggle backed by the owned current-user Run value (UI-010, PERF-005, PRIV-008). Gate: normal launch is visible/full, background launch is hidden/event-driven, toggle round-trip tests pass, and no service/task/admin permission is used.
- [ ] 6.12 Implement overlay visibility toggling, `Esc` hide, active-monitor positioning, and full-workspace expansion without creating a second calculation state (UI-009, MODE-006). Gate: state-parity and repeated show/hide integration tests pass.
- [ ] 6.13 Measure hidden overlay-ready CPU, repaint, disk, network, working set, and explicit Quit cleanup (UI-010, PERF-002, PERF-003, PERF-005). Gate: 30-second trace shows no periodic work and Quit leaves no process or hotkey.

## 7. Persistence, History, Variables, and Memory

- [ ] 7.1 Implement the storage trait and dedicated lazy SQLite worker with bounded request/reply channels (STATE-007, PRIV-001). Gate: domain crates remain SQLite-free and the UI-thread blocking probe stays below budget.
- [ ] 7.2 Create versioned schemas/migrations for history, variables, settings, favorites, and migration log with transactional backup behavior (PRIV-006). Gate: migration tests cover fresh DB, every supported prior version, rollback, and interrupted migration.
- [ ] 7.3 Persist context-complete successful history entries and surface recoverable write failures without losing the current result (STATE-001, STATE-007). Gate: round-trip tests preserve canonical value, exactness, context, snapshots, and versions.
- [ ] 7.4 Implement history grouping, search, pin, copy, edit-and-rerun, count/age retention, and pinned exemptions (STATE-002, PRIV-004). Gate: functional tests pass and 10,000-row search meets 50 ms on the reference device.
- [ ] 7.5 Implement saved-context versus current-context replay for entries referencing variables (STATE-003). Gate: changed-variable fixture exposes both choices and reproduces the original exactly.
- [ ] 7.6 Implement named variable list/edit/copy/delete and explicit overwrite UI on top of engine snapshots (STATE-004). Gate: persistence/restart and cycle-error integration tests pass.
- [ ] 7.7 Implement `M+`, `M-`, `MR`, and `MC` with visible memory state and cross-mode deterministic behavior (STATE-005). Gate: memory command fixtures pass for exact and approximate values.
- [ ] 7.8 Implement private sessions that suppress all session persistence and recent searches/pairs (STATE-006, PRIV-004). Gate: restart test finds no private-session artifacts in DB, logs, or recovery files.
- [ ] 7.9 Implement clear-history confirmation with undo and separate complete local-data deletion (UI-008, PRIV-005). Gate: undo restores only history; full delete verification finds no listed data after restart.
- [ ] 7.10 Implement integrity check, quarantine, recover/export/reset flow, and calculation-first degraded startup (REL-002). Gate: corrupt/truncated/locked storage fixtures never prevent headless calculation.

## 8. Scientific and Programmer MVP

- [ ] 8.1 Implement scientific function dispatch and constants through the approved arbitrary-precision context (MODE-001). Gate: high-precision oracle/golden fixtures cover normal, boundary, and domain cases.
- [ ] 8.2 Implement explicit/global DEG/RAD handling and make angle context visible and historical (MODE-001, UI-005). Gate: degree, radian, explicit-unit, and replay fixtures pass.
- [ ] 8.3 Implement standard/scientific/engineering representation and significant-digit controls as formatting-only state (MODE-002). Gate: retained-value invariance property passes across format switches.
- [ ] 8.4 Implement BIN/OCT/DEC/HEX parsing/display and 8/16/32/64-bit signed/unsigned contexts (MODE-003). Gate: representation matrix covers minimum, maximum, sign-bit, and zero for every word size.
- [ ] 8.5 Implement bitwise operators, shifts, rotates, and documented checked/wrap policy with visible overflow/truncation warnings (MODE-003, MODE-004). Gate: exhaustive 8-bit and property-based wider-word tests pass.
- [ ] 8.6 Implement the keyboard/pointer bit field with atomic multi-base updates and non-color state cues (MODE-005, A11Y-004). Gate: keyboard toggling and accessibility-tree tests pass at all word sizes.
- [ ] 8.7 Verify every Scientific/Programmer on-screen control emits the shared expression/command path (MODE-006). Gate: UI/domain parity fixtures show no duplicated arithmetic behavior.
- [ ] 8.8 Add explicit unsupported-capability messages for complex, matrices, statistics, symbolic math, solvers, and user functions (MODE-007). Gate: unsupported inputs never fall through to misleading partial results.

## 9. Offline Unit Conversion MVP

- [ ] 9.1 Define the versioned unit-catalog schema with stable IDs, dimensions, symbols, localized aliases, region/system, conversion rule, dataset version, and provenance (UNIT-003). Gate: schema/provenance validator rejects missing or duplicate identifiers.
- [ ] 9.2 Implement dimension algebra and multiplicative/affine conversion without converting through binary `f64` (UNIT-001, UNIT-002). Gate: golden/property tests cover inverse conversions, temperature offsets, and incompatible dimensions.
- [ ] 9.3 Populate and review MVP categories: length, area, volume, mass, time, speed, temperature, angle, pressure, energy, power, data size, and data rate (UNIT-001). Gate: catalog completeness and independent factor review are recorded.
- [ ] 9.4 Integrate quantity and `value unit -> unit` grammar with stable source spans and error codes (UNIT-001, UNIT-002). Gate: engine and CLI conversion fixtures pass without a UI dependency.
- [ ] 9.5 Implement localized unit search by name/symbol/alias with explicit ambiguity for regional and decimal/binary units (UNIT-004, UNIT-005). Gate: ambiguity fixtures never auto-select one valid interpretation.
- [ ] 9.6 Implement swap, favorites, and recent pairs with stable IDs and private-session suppression (UNIT-005, STATE-006). Gate: locale change preserves favorites and private restart leaves no recent pair.
- [ ] 9.7 Persist unit dataset version in history and preserve original results across catalog updates (UNIT-003, STATE-001). Gate: version-migration fixture distinguishes stored and newly evaluated results.
- [ ] 9.8 Implement the explicit offline message for currency/crypto and ensure the base dependency graph has no HTTP client (UNIT-006, PRIV-001). Gate: architecture test rejects network crates and offline UI test passes.
- [ ] 9.9 Implement bare data-quantity popular equivalents and Russian aliases for MB/Mbit/MiB while preserving size-versus-rate semantics (UNIT-007, UNIT-008). Gate: exact fixtures for `15 мб`, `15 мб -> бит`, `15 мбит -> бит`, `15 миб`, and `15 Мбит/с` pass in engine, CLI, and overlay.

## 10. Accessibility and Localization Completion

- [ ] 10.1 Audit every component for role/name/value/state/action, relationships, and committed result/error announcements (A11Y-002). Gate: automated semantic-tree snapshot has no unnamed actionable item and Narrator manual script passes.
- [ ] 10.2 Audit deterministic Tab/Shift+Tab order, visible focus, escape behavior, drawers/dialogs, command palette, history, variables, units, and programmer bit field (A11Y-001). Gate: full keyboard journey has no trap or pointer-only command.
- [ ] 10.3 Complete 100/150/200% DPI and minimum-size reflow across every MVP workspace (A11Y-003). Gate: screenshot/layout assertions show no clipped primary control or horizontal primary-action scroll.
- [ ] 10.4 Validate 4.5:1/3:1 contrast, high-contrast tokens, non-color cues, reduced motion, and target sizes (A11Y-004, A11Y-005). Gate: automated contrast report plus manual high-contrast checklist pass.
- [ ] 10.5 Implement English and Russian catalogs for UI, commands, units, errors, and accessibility names with structured placeholders (A11Y-006, A11Y-007). Gate: localization completeness and placeholder-parity commands report zero gaps.
- [ ] 10.6 Implement locale decimal/group normalization and invariant internal/copy representation without ambiguous commas (A11Y-006, EXPR-006). Gate: mirrored English/Russian input and copy fixtures pass.
- [ ] 10.7 Add F1 help generated from grammar, commands, shortcuts, functions, units, limits, exactness, and privacy data sources (UI-003, UI-006). Gate: help-link validator finds no undocumented MVP command or function.

## 11. Reliability, Privacy, and Performance Hardening

- [ ] 11.1 Run sustained lexer/parser/evaluator/storage fuzzing, minimize failures, and add every finding to the normal regression corpus (REL-001). Gate: the release fuzz duration completes with zero unresolved crash, hang, or unbounded allocation.
- [ ] 11.2 Add property and differential suites for arithmetic, scientific functions, formatter round trips, units, fixed-width integers, and state invariants (REL-001). Gate: seeds and oracle/tolerance policy are versioned with results.
- [ ] 11.3 Test missing, read-only, locked, truncated, corrupt, future-version, and interrupted-migration storage states (REL-002, PRIV-006). Gate: calculator startup remains available and no source file is silently overwritten.
- [ ] 11.4 Verify local-only behavior and clipboard intent with network/disk/clipboard instrumentation (PRIV-001, PRIV-002, PRIV-003). Gate: first-launch and 30-second idle traces show zero network, telemetry ID, clipboard read, and idle write.
- [ ] 11.5 Verify retention, private session, full deletion, recovery artifacts, and active-handle closure (PRIV-004, PRIV-005). Gate: forensic test enumerates app-data paths before/after each lifecycle action.
- [ ] 11.6 Optimize and gate cold/warm startup with at least 20 controlled runs (PERF-001). Gate: cold p95 ≤300 ms and warm p95 ≤120 ms with raw evidence.
- [ ] 11.7 Optimize and gate Standard idle working set, private bytes, installed size, and graph-disabled dependency tree (PERF-002). Gate: Standard ≤50 MB hard/≤35 MB target and base install target evidence is recorded.
- [ ] 11.8 Gate idle CPU/GPU, repaint, timers, disk, and network for 30 seconds after quiescence (PERF-003). Gate: CPU ≤0.1% average and no periodic work is present.
- [ ] 11.9 Gate key-to-paint, typical evaluation, UI-thread task duration, 10,000-row search, and shutdown (PERF-004, PERF-005). Gate: every p95/hard threshold passes and no process remains.
- [ ] 11.10 Run the full reproducible release matrix and publish checksums, traces, gate JSON, dependency inventory, and revision identity (REL-003, PRIV-007). Gate: `xtask verify-release` fails closed on missing evidence.

## 12. Windows Packaging and MVP Release Readiness

- [ ] 12.1 Produce a self-contained release x64 portable directory/zip with app icon/version metadata, no console window, notices, SBOM, checksum, and no updater/service (PERF-002, PERF-005, PRIV-007). Gate: it runs on a clean supported Windows VM and leaves no files outside documented app-data paths.
- [ ] 12.2 Decide and verify static versus redistributable MSVC runtime strategy using official redistribution rules (REL-003). Gate: dependency inspection on a clean VM finds no developer-machine DLL leakage.
- [ ] 12.3 Create MSIX packaging with clean install, upgrade, uninstall, local-data preservation policy, and no startup/background declaration (PRIV-005, PRIV-006). Gate: install/upgrade/uninstall VM script passes and matches the portable app semantics.
- [ ] 12.4 Add Windows 11 ARM64 build only after native dependencies and selected renderer pass compile, launch, accessibility, and performance smoke gates (REL-003). Gate: unsupported combinations are not advertised.
- [ ] 12.5 Perform independent UX, math, accessibility, privacy, license, and performance review against all MVP requirement IDs (REL-003). Gate: every finding is fixed, accepted via spec change, or explicitly blocks release.
- [ ] 12.6 Generate release notes and user-facing help that accurately state offline behavior, supported modes, precision/limits, local-data paths, accessibility, known limitations, and excluded features (PRIV-001, MODE-007). Gate: documentation assertions are checked against fixtures/config rather than copied manually.
- [ ] 12.7 Add reversible per-user install/uninstall scripts for the stable `%LOCALAPPDATA%\Programs\LumaCalc` path, Start Menu shortcut, and optional explicitly requested startup registration (PRIV-008). Gate: clean-VM install/uninstall leaves no owned shortcut or startup value and preserves unrelated entries.

## 13. Post-MVP Graphing Change

- [ ] 13.1 Reconfirm base MVP performance/accessibility budgets before enabling graph dependencies in the product build (PERF-002 through PERF-004). Gate: base metrics remain within limits and graph is feature-isolated.
- [ ] 13.2 Implement shared-AST `y=f(x)` evaluation with numeric-context parity and no graph-specific math parser (GRAPH-001). Gate: calculator/graph parity fixtures pass.
- [ ] 13.3 Implement bounded adaptive sampling for up to five functions with discontinuity detection and explicit limits (GRAPH-002). Gate: discontinuity, oscillation, domain-error, and worst-case fixtures complete within bounds.
- [ ] 13.4 Implement generation-based cancellation and bounded lazy graph workers for edit/pan/zoom (GRAPH-003). Gate: rapid-interaction test rejects every stale frame and shutdown leaves no worker.
- [ ] 13.5 Implement plot viewport, show/hide, non-color identifiers, range, auto-fit, pan, zoom, reset, cursor, and keyboard trace (GRAPH-002, GRAPH-004). Gate: pointer and keyboard integration tests produce equivalent viewport state.
- [ ] 13.6 Implement textual range/discontinuity summary and keyboard-navigable sampled-point table (GRAPH-005). Gate: Narrator can inspect all functions without using the canvas.
- [ ] 13.7 Gate graph memory ≤80 MB, ordinary final refresh ≤100 ms, responsive interaction, and zero graph work when unopened (PERF-002 through PERF-004). Gate: controlled benchmark evidence passes without weakening base budgets.
- [ ] 13.8 Add explicit unsupported handling for 3D, implicit, polar, parametric, symbolic, solver, intersection, and export requests (GRAPH-006). Gate: deferred-scope fixtures preserve input and never claim partial support.
