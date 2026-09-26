## ADDED Requirements

### Requirement: PERF-001 Measured startup budget
Release builds on the reference Windows 11 test device MUST reach a focused, interactive expression field within 300 ms at cold-start p95 and 120 ms at warm-start p95 over at least 20 runs. Measurement methodology and raw results SHALL be retained with the build.

#### Scenario: Startup regression gate
- **WHEN** a release candidate exceeds either startup hard limit under the controlled benchmark
- **THEN** the release gate fails or an approved spec change documents the new budget and evidence

### Requirement: PERF-002 Memory and installed-size budget
After 30 seconds idle in the Standard workspace, working set MUST be at most 50 MB with a 35 MB target. The graph workspace MUST remain at most 80 MB. The installed base application MUST target 25 MB and SHALL report any hard dependency that prevents the target.

#### Scenario: Idle memory measurement
- **WHEN** a clean release build is opened to Standard mode and left idle for 30 seconds
- **THEN** measured working set and private bytes are recorded and the 50 MB hard limit is enforced

### Requirement: PERF-003 True idle behavior
With no animation, input, or foreground work, average app CPU MUST be at most 0.1%, the app SHALL schedule no periodic UI repaint or maintenance timer, and it SHALL perform no network or disk writes.

#### Scenario: Thirty-second idle trace
- **WHEN** the app is traced for 30 seconds after quiescence
- **THEN** the trace contains no periodic repaint loop, no background network activity, and no persistence write

### Requirement: PERF-004 Interaction latency budget
Key-to-paint latency MUST be at most 16 ms p95, typical evaluation at most 5 ms p95, UI-thread tasks at most 50 ms, search across 10,000 representative history rows at most 50 ms, and final ordinary graph refresh at most 100 ms on the reference device.

#### Scenario: UI-thread blocking
- **WHEN** parsing, persistence, or graphing is predicted or observed to exceed 50 ms
- **THEN** it runs as cancellable background work and publishes results only if still current

### Requirement: PERF-005 No residual process
Invoking the explicit Quit command SHALL terminate the process and release resources. Hiding the quick overlay MAY keep the calculator process resident only when overlay-ready mode is active. The base product SHALL install no Windows service or scheduled task, and optional startup registration SHALL require explicit user action.

#### Scenario: Quit application
- **WHEN** the user invokes Quit after background work was started
- **THEN** background work is cancelled or joined within the shutdown budget, the global hotkey is unregistered, and no calculator process remains

#### Scenario: Hidden overlay idle
- **WHEN** overlay-ready mode is active and the window is hidden for 30 seconds
- **THEN** the process performs no periodic repaint, disk write, or network request and remains within the idle CPU budget

#### Scenario: Background launch
- **WHEN** the executable is started with `--background`
- **THEN** no window is shown until the global hotkey is invoked and the process performs no polling or periodic rendering while hidden

### Requirement: REL-001 Panic-free untrusted input
Arbitrary expression and pasted input MUST NOT cause a panic, crash, unbounded allocation, stack overflow, or hang. Parser/evaluator fuzzing and property tests SHALL run in CI with persisted regression seeds.

#### Scenario: Fuzz regression
- **WHEN** CI replays all saved crashing or timeout seeds
- **THEN** each seed returns a bounded success or structured error without process failure

### Requirement: REL-002 Corrupt-state recovery
The app SHALL start when settings or history storage is missing, partially written, incompatible, or corrupt. It SHALL isolate affected data, preserve recoverable records, and offer a reset or export path without blocking calculation.

#### Scenario: Corrupt history database
- **WHEN** the history store fails its integrity check at startup
- **THEN** calculation remains available, the damaged file is quarantined, and the user is offered recovery or reset with no silent overwrite

### Requirement: REL-003 Reproducible release gates
Formatting, linting, unit, golden, property, integration, accessibility-smoke, localization, packaging, dependency-license, security audit, and performance gates SHALL be documented and automated for supported targets.

#### Scenario: Clean checkout verification
- **WHEN** CI builds a tagged source revision with its lockfile
- **THEN** it produces checksummed artifacts and a machine-readable report tying every gate to that revision
