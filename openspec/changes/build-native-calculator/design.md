## Context

The repository contains no application code, compatibility constraints, or user data. The first change therefore has to establish both the product contract and the architecture that future AI-assisted changes will follow.

The product is a Windows-first, offline desktop calculator. Its value is not a large checklist alone: it must remain fast to launch, quiet while idle, predictable with a keyboard, understandable when mathematics is approximate, and usable with accessibility tooling. A browser runtime, background service, cloud account, or telemetry pipeline would work against those goals.

The technology assessment was refreshed on 2026-07-16. The planning baseline is Slint 1.17.x on stable Rust; exact crate and toolchain versions will be locked during bootstrap rather than floated in build scripts. Relevant primary documentation is retained here for future drift checks:

- Slint desktop support: https://docs.slint.dev/latest/docs/slint/guide/platforms/desktop/
- Slint backends and renderers: https://docs.slint.dev/latest/docs/slint/guide/backends-and-renderers/backends_and_renderers/
- Slint release stream: https://github.com/slint-ui/slint/releases
- Slint licensing: https://slint.dev/pricing
- Iced project status: https://github.com/iced-rs/iced
- egui goals and non-goals: https://github.com/emilk/egui
- Qt Windows deployment: https://doc.qt.io/qt-6/windows-deployment.html
- WinUI 3 architecture: https://learn.microsoft.com/windows/apps/winui/winui3/
- Windows Rust bindings: https://github.com/microsoft/windows-rs
- Arbitrary-precision candidate: https://docs.rs/astro-float/latest/astro_float/

The published performance numbers in the capability specs are product budgets, not claims about framework benchmark results. The technology spike must produce local evidence before implementation expands.

## Goals / Non-Goals

**Goals:**

- Ship a polished native utility for Windows 10/11 without HTML, CSS, JavaScript, WebView, Electron, or a resident background process.
- Make calculation semantics independent from UI rendering and fully testable headlessly.
- Support Standard, Scientific, Programmer, variables, searchable history, and offline units in the MVP, with graphing as the next bounded increment.
- Meet explicit startup, memory, idle, latency, accessibility, localization, resilience, privacy, and package-size release gates.
- Give AI agents small, typed module boundaries, stable requirement/error identifiers, machine-readable fixtures, and deterministic verification commands.
- Keep a credible exit path if the selected Rust GUI toolkit fails a measured release gate.

**Non-Goals:**

- Pixel-for-pixel imitation of Windows Calculator or dependence on undocumented Windows UI internals.
- Cross-platform release parity in the initial product, even though the domain crates remain portable.
- Live currency/crypto, accounts, cloud sync, telemetry, embedded AI chat, plugins, scripting, OCR, symbolic algebra, matrices, statistics, equation solving, or 3D plots.
- A custom windowing, text, layout, accessibility, or rendering toolkit built directly on Win32/Direct2D.
- Background updates, Windows services, widgets, or mandatory startup registration in the MVP. A user-visible in-process global-hotkey mode is in scope for the quick overlay.

## Decisions

### D1. Use stable Rust and Slint for the product baseline

Use a Cargo workspace with stable Rust and Rust 2024 edition. Pin the compiler in `rust-toolchain.toml`, commit `Cargo.lock`, and update either only through reviewed dependency changes. Use Slint 1.17.x with its compiled declarative `.slint` language and the `winit` backend. The UI is native rendered and does not contain a browser engine.

Why:

- Rust provides a small native process, strong memory safety, explicit ownership across worker/UI boundaries, mature property/fuzz testing, and one language for domain and Windows adapter code.
- Slint provides a retained declarative UI, compiled bindings, live design preview, custom styling, and multiple renderer choices without moving the product to web technology.
- `.slint` files are concise enough for design iteration while typed Rust callbacks keep business behavior out of view markup.

Alternatives considered:

| Alternative | Decision |
|---|---|
| Iced | Reject for baseline. Its Elm-style architecture is attractive, but its public project status and accessibility maturity create greater consumer-UX risk. Revisit only after a separate spike. |
| egui/eframe | Reject for product UI. Immediate mode is excellent for tools, but native look is a non-goal of egui and frame-driven layout is a worse fit for strict desktop accessibility and idle budgets. |
| C++20 + Qt Quick/QML | Keep as the only planned fallback. It has mature controls, accessibility, tooling, and Windows deployment, but carries a larger runtime/deployment surface and licensing compliance work. |
| C# or C++ + WinUI 3 | Reject for baseline. It is Windows-native and visually strong, but couples the app to Windows App SDK packaging/runtime and does not improve the headless engine. It remains a platform-specific option only if product priorities change to system integration over footprint. |
| UWP | Reject. It is not the active path for new unrestricted desktop utilities. |
| `windows-rs` + Direct2D/DirectWrite | Reject as a UI toolkit. It would require implementing layout, focus, DPI, IME, themes, input, UI Automation, and test infrastructure. Use `windows-rs` only behind narrow adapters. |

### D2. Select the production renderer by an early measured spike

Build the same vertical slice with three Slint profiles:

1. `winit-software`: provisional production candidate for the dense PowerToys Run-style UI.
2. `winit-femtovg`: GPU-rendered fallback candidate.
3. `winit-femtovg-wgpu`: Direct3D-capable compatibility candidate if required by graphing or graphics compatibility.

Do not enable Skia in the baseline because its dependency and package footprint work against the product target. The initial live x64 release comparison on 2026-07-16 rendered the compact Russian result-list slice equivalently while `winit-software` used approximately 25 MiB working set versus approximately 115 MiB for `winit-femtovg`; private bytes were approximately 6 MiB for software. This single-run evidence selects software for the current normal build, but does not replace the required 20-run, DPI, accessibility, latency, and compatibility matrix. Promote a GPU renderer only if it passes the same UX gates and a measured feature or compatibility need justifies the resource increase.

The spike screen must include the real expression editor, result typography, adaptive keypad, history drawer, Russian text, error span, focus visuals, and theme tokens. All candidate profiles are tested at 100/150/200% DPI, light/dark/high contrast, keyboard-only, Narrator, resize stress, remote/virtualized graphics where available, and with no animation.

Record for at least 20 release runs per profile:

- cold and warm time to a focused input;
- working set/private bytes at launch and after 30 seconds idle;
- idle CPU/GPU, repaint events, disk and network activity;
- package and executable size;
- key-to-paint latency and resize frame pacing;
- text clarity for English/Russian and accessibility-tree correctness.

Decision rule:

- Choose the smallest profile that passes every hard UX/accessibility gate and every hard resource budget.
- A screen-reader/focus defect without a safe, bounded adapter is an immediate Slint blocker.
- A renderer may miss one provisional target only with a documented optimization experiment; it may not miss a hard limit.
- If neither renderer passes, stop feature work and implement the same vertical slice in Qt Quick/QML. Do not respond by building a custom Win32 toolkit.

Only one production renderer profile is compiled into the normal release unless a measured compatibility case proves that a fallback renderer is required.

### D3. Keep a strict layered Cargo workspace

Planned structure:

```text
apps/
  calculator-desktop/       Slint window, composition root, app lifecycle
crates/
  calc-syntax/              lexer, Pratt parser, AST, spans, grammar fixtures
  calc-numbers/             numeric values, precision, evaluation, formatting
  calc-units/               dimensions and versioned offline unit catalog
  calc-state/               commands, app model, history/variable contracts
  calc-storage/             SQLite worker, migrations, recovery, retention
  calc-graph/               bounded sampler and graph-domain models
  calc-platform-windows/    feature-gated narrow Windows adapters
  calc-test-support/        fixture loaders, deterministic clocks, generators
ui/
  components/               reusable Slint controls with accessibility metadata
  themes/                   tokens for light, dark, high contrast, density
fixtures/
  expressions/ units/ errors/ history/ localization/
benchmarks/
  startup/ interaction/ memory/ history/ graph/
packaging/
  portable/ msix/
```

Dependency direction is inward: UI and platform adapters depend on domain crates; domain crates never depend on Slint, SQLite, Windows APIs, filesystem paths, clocks, or localization catalogs. `calc-state` owns typed commands and immutable snapshots; the desktop app maps them to Slint properties and callbacks.

### D4. Use `fend-core` behind a bounded calculator adapter

Use `fend-core` 1.5.x as the headless expression and general unit-aware engine behind `calc-engine`. It is a pure Rust arbitrary-precision calculator library with scientific functions, variables, units, conversion, preview interruption, and no UI dependency. This substantially reduces correctness risk and implementation time while preserving a small native process. The application does not configure an exchange-rate provider, so the base engine remains offline.

`calc-engine` owns the public contract: locale/Russian alias normalization, `->`/localized conversion syntax, input length and elapsed-work limits, stable application error codes, result exactness metadata, history snapshots, and popular data-unit equivalents. No Slint type crosses this boundary. Engine-specific messages and syntax are not exposed as stable product APIs.

A custom lexer/Pratt parser remains a fallback only if fixture evidence proves that `fend-core` cannot satisfy required semantics without unsafe preprocessing. This decision is reversible because all calls go through the `CalculatorEngine` trait and golden fixtures.

### D5. Preserve arbitrary precision through the engine boundary

Retain canonical expression/result strings and structured metadata rather than collapsing values to primitive `f64`. `fend-core` owns arbitrary-precision arithmetic and unit algebra. The adapter may use fixed decimal arithmetic only for the small, independently tested table that generates popular data-size equivalents; it must preserve exact bit counts and decimal-versus-binary prefixes.

User-visible precision is bounded and separate from display formatting. Output length, exponent, factorial, recursion, and elapsed-work limits remain explicit. Raw `NaN`, infinity, or engine panic text is mapped to stable domain errors. Programmer fixed-width operations remain a separate typed module if the engine's semantics do not match the approved wrap/checked policy.

### D6. Make state transitions explicit and UI bindings thin

The desktop layer dispatches typed commands such as `EditExpression`, `Evaluate`, `ChangeAngleUnit`, `ReplayHistory`, and `ToggleBit`. A reducer/service layer produces a new serializable `AppSnapshot` plus bounded effects. Slint receives display models and emits callbacks; it does not parse expressions, implement math, query SQLite, or decide persistence.

Benefits for AI development:

- state changes are reviewable as typed inputs/outputs;
- reducers run without a window;
- fixture snapshots expose accidental semantic drift;
- multiple agents can change UI and domain code with less overlap;
- stale background results are rejected with monotonically increasing generation IDs.

### D7. Use SQLite through a dedicated storage worker

Use bundled SQLite through `rusqlite` unless the package-size spike disproves the target. One storage worker owns the connection. UI code sends bounded typed requests and receives results asynchronously; there is no async runtime or connection pool.

Core tables:

- `history_entry`: expression, canonical value payload, display result, exactness, context JSON/version, variable snapshot, unit dataset version, created time, session, pinned flag;
- `variable`: stable name, canonical value payload, modified time;
- `setting`: schema-versioned key/value records;
- `conversion_favorite`: source/target stable unit IDs and ordering;
- `migration_log`: schema version, status, app version, checksum.

Use transactions, foreign keys, integrity checks, busy timeout, and versioned migrations. Writes occur after relevant user actions, never on a periodic timer. Before destructive migration, copy or SQLite-backup the database. On corruption, quarantine the file and keep the headless calculator available.

A JSON/append-only store was rejected because indexed search, transactional migrations, snapshots, pinning, and corruption recovery would recreate database concerns in application code.

### D8. Avoid a general async runtime

The base product has no network and very little concurrent I/O. Use the Slint event loop plus bounded standard channels and dedicated threads:

- one lazy storage worker;
- one bounded compute worker for previews/heavy evaluation;
- a graph worker pool of at most `min(2, available_parallelism - 1)` created only when graphing is opened.

Cancellation uses generation tokens plus cooperative checks at parser/evaluator/sampler limits. Background workers publish immutable results back through the UI event-loop bridge. Shutdown cancels and joins workers; no process survives the final window.

Tokio and a resident general-purpose thread pool are rejected until a separately specified feature demonstrates a need.

### D9. Use one dark Fluent visual system across the full calculator and quick overlay

Use Slint design tokens for semantic colors, typography, spacing, radii, elevation, focus, motion, and density. Ordinary launch opens a complete dark calculator workspace inspired by the information architecture of Windows Calculator: a minimal draggable strip, collapsible navigation rail, compact expression/result area, keypad or mode tool, and optional history panel. The rail groups Standard, Scientific, Programmer, and Date calculation separately from offline converter categories and Settings. It uses original icons, text, and implementation rather than copying Microsoft assets. Application name, active-mode title, and engine metadata appear at most once and only when they help orientation; implementation phrases such as the arithmetic backend or precision strategy never appear in the primary workflow.

The compact overlay retains the rapid PowerToys Run interaction hierarchy while using the same dark tokens: one dominant full-width expression field, a hairline divider, and a vertically scanned list of results. No elastic spacer may push the editor away from the leading edge. Overlay height is derived from the visible error/result/suggestion rows within a documented minimum and maximum instead of reserving full-workspace height. Each result row has one compact semantic icon, a primary value, a secondary label or explanation only when it adds user meaning, and a single hover/selection state. Prefer `Segoe UI Variable` with OS fallbacks. Standard pointer targets are at least 40 logical pixels in dense lists and 44 logical pixels for primary actions. Animations are short, interruptible, and absent under reduced motion or idle. The production software-rendered surface remains opaque because visual QA found background bleed harmed compact-mode readability; acrylic, Mica, blur, continuous animation, and renderer-specific effects remain excluded until independently measured.

Consistency rules are release requirements: one 4/8-pixel spacing scale, one radius family (`4/8/12`), one restrained blue-violet focus/selection accent per theme, no traffic-light chrome, no decorative neon/glow, no capsule or card collection where a flat row works, no more than three type scales in the overlay, and shared row states across navigation, conversions, suggestions, history, and commands. Expand, Hide, and Quit remain keyboard-accessible secondary actions in a compact trailing command group; the window header does not compete with the expression field.

Layout states:

- compact: dominant editor plus a flat result/suggestion list and a minimal status/footer row;
- regular: collapsible navigation plus keypad or converter workspace;
- expanded: persistent navigation, optional history/variables panel, and mode-specific tools;
- graph: expression list, viewport, and accessible data panel.

Custom controls must define focus order and accessibility role/name/value/action at creation time. Visual validation SHALL compare the compact overlay with the PowerToys Run reference for hierarchy and the expanded workspace with Windows Calculator for navigation clarity, without copying logos or assets. Both views SHALL demonstrate the same tokens and control states. The graph always has a textual/table alternative.

### D10. Treat localization and shortcuts as data

Ship English and Russian Fluent-style message catalogs. Error codes map to localized templates with structured arguments. Unit IDs and command IDs remain invariant; localized names and aliases are presentation/search data.

Store shortcuts in a single machine-readable command registry consumed by the command palette, tooltips, help, and tests. Resolve conflicts at startup in debug/test builds. Locale normalization happens before lexing and is covered by mirrored locale fixtures.

### D11. Use narrow Windows adapters only where Slint is insufficient

`calc-platform-windows` may wrap `windows-rs` APIs for system theme/high contrast/reduced motion, app-data paths, window chrome details, taskbar metadata, and ETW markers. Each adapter has a trait and a fake for tests. Clipboard and basic windowing remain with Slint when its behavior passes acceptance tests.

Do not let raw Win32 handles, HRESULTs, COM types, or OS strings enter domain crates. Do not add Jump Lists, file associations, or app services without a new OpenSpec capability.

The quick overlay uses a narrow `calc-platform-windows` adapter. A dedicated thread calls `RegisterHotKey(NULL, id, MOD_WIN | MOD_ALT | MOD_NOREPEAT, VK_SPACE)` and blocks in `GetMessageW`; on `WM_HOTKEY` it posts a typed toggle request through `slint::invoke_from_event_loop`. It performs no polling. The adapter unregisters the hotkey during shutdown and reports registration conflicts to the UI. Slint remains responsible for rendering; the selected winit window is configured undecorated and always-on-top for overlay mode. Closing/hiding the overlay keeps the event-driven listener alive, while an explicit Quit command terminates the event loop and process.

The full and overlay workspaces share one app model and engine. Overlay mode is 680x380 logical pixels, opens near the active monitor's top center, focuses and selects the expression field, hides secondary navigation, and exposes compact Expand, Hide, and Quit actions. `Win+Alt+Space` toggles visibility when already in overlay mode; `Esc` hides it. No expression is evaluated merely by showing the overlay.

Ordinary interactive launch opens the full workspace. `--background` starts the same executable with no visible window, registers the hotkey, and blocks only on the native event loops. A hotkey request from this state opens the compact overlay. The background state performs no polling, periodic repaint, persistence write, or network operation.

### D12. Package self-contained artifacts with no resident updater

Primary MVP artifact: signed portable x64 zip containing the application and required notices, with no installer-side service. A user-level install script may copy the executable to a stable `%LOCALAPPDATA%\Programs\LumaCalc` path and create a Start Menu shortcut. Secondary artifact: MSIX for clean installation/uninstall and future Store distribution. Add Windows 11 ARM64 only after the renderer, native dependencies, and CI runner pass the same gates. Windows 10 ARM64 is not claimed by the Slint support baseline.

Launch-at-login is an explicit Settings toggle implemented through a quoted per-user `HKCU\Software\Microsoft\Windows\CurrentVersion\Run` value that starts the stable executable with `--background`. It requires no elevation, is disabled by default, reports failures, and can be removed from the app or uninstall script. The app never creates a service or scheduled task.

Evaluate static MSVC runtime linkage during the packaging spike; do not copy arbitrary runtime DLLs from a developer machine. Generate SHA-256 checksums and an SBOM. Auto-update is deferred; distribution updates replace the app explicitly and migrate only versioned local data.

The Slint Royalty-Free Desktop license is the default planning assumption for a proprietary desktop release and requires the applicable attribution/notices. Before public distribution, the owner must choose Royalty-Free terms, GPLv3 distribution, or a commercial Slint license and approve all resulting obligations.

### D13. Make performance evidence a first-class artifact

Add a small benchmark harness and Windows measurement scripts. `PERF_REFERENCE.md` records exact hardware, OS build, power mode, display scale, Defender state, build revision, renderer, run count, and measurement commands. Benchmarks produce JSON plus trace files so an AI-generated summary never replaces evidence.

Gates map directly to `PERF-*` requirements. Fast pull-request checks run deterministic microbenchmarks with generous regression thresholds; controlled nightly/release jobs own cold-start, working-set, ETW, Narrator/manual, and package measurements. A budget change requires an OpenSpec modification, not a loosened assertion hidden in code.

### D14. Use a layered verification strategy

- Unit tests: lexer, parser, numeric promotion, rounding, formatting, units, reducers, migrations.
- Golden fixtures: expressions, values, exactness, contexts, error codes/spans, locale variants.
- Property tests: arithmetic identities within declared domains, parse/format round trips, unit inverse conversions, state invariants.
- Differential tests: compare selected scientific fixtures with an independent high-precision oracle in development tooling, never at runtime.
- Fuzzing: lexer/parser/evaluator/unit parser/storage deserializer with saved seeds and time/memory limits.
- UI component tests: focus order, callbacks, adaptive states, themes, command registry.
- Windows integration: DPI, clipboard intent, app-data path, close/shutdown, package install/uninstall.
- Accessibility: automated semantic-tree smoke tests plus Narrator keyboard scripts and a manual release checklist.
- Performance: startup, idle, memory, interaction, history search, graph sampling, installed size.

The baseline CI gate is `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `cargo test --workspace --all-features`, fixture validation, OpenSpec validation, dependency/license audit, and release build. Fuzz, full performance, accessibility, and packaging run on scheduled or release Windows workers.

### D15. Constrain AI-assisted implementation

Every implementation task must:

1. reference one or more requirement IDs;
2. state the files/modules it may change;
3. introduce or update a fixture/test before being marked complete;
4. run the narrowest relevant test and the current workspace gate;
5. record measured evidence for budgets rather than claim success from code inspection;
6. stop and update OpenSpec if observable behavior changes.

Prefer one capability per follow-up OpenSpec change after the bootstrap. Keep generated code, unsafe blocks, dependency additions, schema migrations, and Windows calls under explicit human/reviewer inspection. No AI agent may silently relax a performance limit, broaden network access, or alter numeric semantics to make tests pass.

## Risks / Trade-offs

- **[Slint desktop maturity]** Some advanced desktop integration and custom accessibility paths may be less mature than Qt. → Run the real vertical-slice gate before domain expansion; keep UI/domain boundaries portable; use Qt Quick/QML as the defined fallback.
- **[Renderer compatibility]** FemtoVG may depend on graphics-driver behavior while WGPU can add dependencies and size. → Measure both, compile only the selected profile, retain a compatibility build only with evidence.
- **[Accessibility gap in custom rendering]** A beautiful custom control can be invisible to Narrator. → Treat semantic metadata and keyboard behavior as component definition-of-done; fail the spike on an unfixable gap.
- **[Arbitrary-precision complexity]** Precision, rounding, cancellation, and transcendental behavior can produce subtle defects or large allocations. → Hide the numeric backend, bound context, use oracle/golden/property tests, and mark approximation explicitly.
- **[Pure-Rust numeric library maturity]** The preferred numeric candidate may have lower ecosystem maturity than GMP-backed alternatives. → Complete the numeric spike before grammar freeze and retain a backend trait; compare with `rug` as a development oracle, not an automatic production dependency.
- **[SQLite package cost]** Bundled SQLite increases binary size. → Measure it in the vertical slice; keep the storage trait replaceable, but prefer proven recovery/transactions unless the hard size budget fails.
- **[Over-scoped first release]** Scientific, Programmer, units, variables, and history can delay polish. → Ship only after the vertical slice; sequence capabilities and keep graphing post-MVP. Standard mode remains usable at every milestone.
- **[Performance targets vary by hardware]** Startup and memory values are not universal. → Define and version the reference device/methodology, retain raw traces, and separately run low-end smoke tests.
- **[License choice]** Slint and redistributed libraries impose distribution obligations. → Select the product license before public binaries; automate SBOM/notices and block unreviewed licenses.
- **[Windows-only assumptions leak inward]** Platform APIs can make future portability and headless tests difficult. → Enforce Cargo dependency direction and adapter traits in architecture tests/review.
- **[AI-generated semantic drift]** An agent may duplicate math in UI, change `%` behavior, or loosen limits. → Use stable IDs, centralized fixtures, module ownership, OpenSpec deltas, and independent review before integration.

## Migration Plan

This is a greenfield build, so migration means staged commitment rather than production data conversion.

1. **Foundation:** pin Rust/OpenSpec tooling, create workspace boundaries, fixture schemas, CI gates, and architecture tests.
2. **Technology spike:** build the real native vertical slice in both renderer profiles and capture performance/accessibility evidence. Select Slint renderer or activate the Qt fallback before feature growth.
3. **Numeric spike:** validate exact integer/rational behavior, arbitrary-precision science, limits, error/exactness propagation, and binary cost; freeze grammar v1 and numeric context v1.
4. **Headless engine:** deliver parse → evaluate → format with CLI/fixture harness, fuzzing, and no GUI dependency.
5. **MVP vertical product:** deliver editor → result → copy → history with theme, DPI, keyboard, localization, accessibility, shutdown, and persistence recovery.
6. **MVP capabilities:** add variables/memory, Scientific, Programmer, offline units, and command palette as small requirement-linked changes.
7. **Hardening and packaging:** complete controlled performance traces, Narrator/manual QA, migration tests, SBOM/notices, portable x64, and MSIX.
8. **Post-MVP graphing:** enable `calc-graph` and graph UI only after all base gates remain green.

Rollback strategy:

- Until the renderer gate is approved, no product feature may depend directly on renderer-specific APIs.
- If Slint fails, retain domain crates and fixtures, replace only `apps/calculator-desktop` and `ui/` with the Qt spike, and re-run every UI/performance gate.
- Storage migrations create backups and advance schema version only on successful transaction commit; failed migrations quarantine the attempted database and keep calculation available.
- Dependency/toolchain updates are isolated changes with a committed lockfile diff and can be reverted without changing persisted schemas.

## Open Questions

- Product name, icon, and signing identity are intentionally unresolved; they affect packaging assets but not architecture.
- The owner must choose the final Slint distribution/license model before public release.
- The exact Windows 10 minimum build should be selected from supported OS policy and tested hardware before packaging; the architectural baseline is Windows 10 x64 and Windows 11 x64/ARM64 as documented by Slint.
- The renderer and numeric backend remain provisional until their mandatory spikes produce evidence.
- Whether graphing ships as version 1.1 or a later optional workspace depends on MVP resource and accessibility results; it is not allowed to delay the base calculator.
