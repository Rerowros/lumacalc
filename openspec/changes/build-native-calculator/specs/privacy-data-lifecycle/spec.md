## ADDED Requirements

### Requirement: PRIV-001 Local-only base product
All evaluation, conversion, graphing, settings, variables, and history processing SHALL occur on the local device. The base executable SHALL NOT initiate network requests.

#### Scenario: Offline operation
- **WHEN** the device has no network connectivity
- **THEN** every MVP capability remains functional without degraded calculation behavior

### Requirement: PRIV-002 No telemetry by default
The first release SHALL contain no analytics or telemetry pipeline. Any future crash-reporting capability MUST be opt-in and show the user the data before transmission.

#### Scenario: First launch
- **WHEN** the app is launched for the first time
- **THEN** it creates no telemetry identifier and sends no analytics or crash-reporting request

### Requirement: PRIV-003 Clipboard minimization
The app SHALL read clipboard contents only after an explicit paste action and SHALL write only after an explicit copy action. Clipboard contents MUST NOT be retained in history unless the resulting expression is explicitly evaluated and history is enabled.

#### Scenario: Clipboard remains untouched
- **WHEN** the app is idle or only opened
- **THEN** it neither reads nor writes clipboard data

### Requirement: PRIV-004 Bounded local retention
Users SHALL be able to configure history retention by age and count, pin exempt entries, inspect the storage location and approximate size, and disable persistence with a private session.

#### Scenario: Retention cleanup
- **WHEN** the configured count limit is exceeded
- **THEN** the oldest unpinned entries are removed in a non-blocking maintenance action triggered by a relevant write, not by a periodic idle timer

### Requirement: PRIV-005 Complete local-data deletion
The app SHALL provide one explicit action to delete history, variables, favorites, recent pairs, settings, caches, and recovery files. It SHALL list the affected categories before confirmation and verify deletion after closing active handles.

#### Scenario: Delete all local data
- **WHEN** the user confirms deletion of all local data
- **THEN** the listed data is removed, defaults are restored, and a restart does not restore deleted records

### Requirement: PRIV-006 Versioned storage and migrations
Persistent schemas SHALL be versioned. Migrations MUST be transactional, reversible when practical, backed up before destructive transformation, and tested from every supported prior version.

#### Scenario: Migration failure
- **WHEN** a storage migration fails
- **THEN** the previous data remains recoverable, the app starts in a safe degraded mode, and no partial schema is treated as current

### Requirement: PRIV-007 Dependency and license disclosure
Release artifacts SHALL include notices required by Slint and all other redistributed dependencies. Dependency versions, licenses, and checksums SHALL be recorded in an SBOM and checked against the chosen distribution model.

#### Scenario: Release license gate
- **WHEN** a dependency introduces an incompatible or unreviewed license obligation
- **THEN** packaging fails until the dependency is replaced or the distribution decision is explicitly approved

### Requirement: PRIV-008 Explicit reversible launch-at-login
Launch-at-login SHALL be disabled by default and SHALL require an explicit user action. Enabling it MAY create only a per-user registry value that launches the same executable in `--background` mode. Disabling or uninstalling SHALL remove that owned value without modifying unrelated startup entries.

#### Scenario: Enable launch-at-login
- **WHEN** the user explicitly enables launch-at-login in Settings
- **THEN** the app creates a quoted current-user startup command, reports success or failure, and does not request administrator rights

#### Scenario: Disable launch-at-login
- **WHEN** the user disables launch-at-login
- **THEN** the app removes only its own startup value and the next sign-in does not launch the calculator
