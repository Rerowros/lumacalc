## Why

Windows needs a calculator that combines the speed and footprint of a small native utility with the discoverability, polish, keyboard flow, and advanced workflows usually found in much heavier applications. Starting from an empty repository makes it possible to establish measurable product contracts and an AI-friendly architecture before UI code creates lock-in.

## What Changes

- Build an offline-first Windows 10/11 desktop calculator with a single expression language shared by Standard, Scientific, Programmer, conversion, and graphing workspaces.
- Make the main workflow keyboard-first while retaining an adaptive pointer/touch keypad, command palette, searchable history, variables, and explicit context controls.
- Add a compact always-on-top quick overlay opened system-wide with `Win+Alt+Space`, focused for immediate typing and expandable into the full workspace.
- Make ordinary launch open a complete dark calculator workspace with a collapsible Windows Calculator-style navigation rail for calculator modes, offline converter categories, and settings; keep the quick overlay as a separate compact state.
- Offer an explicit user-level launch-at-login setting so the event-driven quick overlay can remain available after sign-in without a Windows service, scheduled task, administrator rights, or implicit registration.
- When a data quantity such as `15 MB`, `15 мб`, or `15 MiB` has no explicit target, show useful bit/byte equivalents while preserving decimal-versus-binary and size-versus-rate semantics.
- Separate a deterministic, headless Rust calculation engine from a thin native UI so semantics can be tested without rendering.
- Establish local-only persistence, recovery, privacy, accessibility, localization, packaging, and resource budgets as release requirements.
- Deliver Standard, Scientific, Programmer, history, variables, and offline unit conversion in the MVP; deliver graphing as the first post-MVP capability on the same engine.
- Run a native-renderer technology spike before feature expansion, with an explicit fallback decision if Slint fails resource, text, or accessibility gates.
- Exclude live rates, cloud accounts/sync, embedded AI chat, plugins/scripts, symbolic algebra, 3D graphs, telemetry, background services, and web runtimes from the initial product. The optional overlay listener is an in-process event-driven hotkey loop, and launch-at-login is reversible and opt-in.

## Capabilities

### New Capabilities

- `expression-evaluation`: Grammar, numeric semantics, formatting, precision, deterministic errors, safety limits, and cancellation.
- `calculator-workspace`: Adaptive window, editor/result/keypad flow, command palette, themes, modes, copy/paste, undo, and shortcut behavior.
- `scientific-programmer`: Scientific functions, angle context, integer bases, word sizes, bitwise operations, bit editing, and overflow behavior.
- `unit-conversion`: Versioned offline dimensions, affine and multiplicative conversions, aliases, favorites, and ambiguity handling.
- `history-variables`: Searchable local history, context snapshots, memory registers, named variables, replay semantics, and private sessions.
- `graphing-workspace`: Bounded, cancellable 2D function plotting with pan/zoom/trace and an accessible tabular alternative; scheduled after MVP.
- `accessibility-localization`: Keyboard navigation, Windows accessibility semantics, scaling, contrast, reduced motion, and locale-aware input/output.
- `performance-reliability`: Startup, memory, idle CPU, latency, package size, resilience, fuzzing, and regression gates.
- `privacy-data-lifecycle`: Local-only data, no implicit network or telemetry, bounded storage, data deletion, and corrupt-data recovery.

### Modified Capabilities

None. This is a greenfield repository with no existing specifications.

## Impact

- Introduces a Rust workspace, Slint UI assets, native Windows integration adapters, tests/fixtures, benchmarks, and Windows packaging automation.
- Adds versioned dependencies for parsing/evaluation, decimal/integer math, local persistence, localization, tracing, property tests, fuzzing, and Slint.
- Defines release support for Windows 10 x64 and Windows 11 x64/ARM64, subject to the technology-spike results.
- Adds OpenSpec as the source of truth for behavioral contracts and staged AI-assisted implementation.
