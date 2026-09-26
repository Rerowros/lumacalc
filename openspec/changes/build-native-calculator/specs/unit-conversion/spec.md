## ADDED Requirements

### Requirement: UNIT-001 Offline dimensional conversion
The system SHALL convert values locally between compatible units for length, area, volume, mass, time, speed, temperature, angle, pressure, energy, power, data size, and data rate. A network connection SHALL NOT be required.

#### Scenario: Multiplicative conversion
- **WHEN** the user evaluates `1 km -> m`
- **THEN** the exact result is `1000 m`

#### Scenario: Affine conversion
- **WHEN** the user evaluates `0 C -> F`
- **THEN** the result is `32 F` using the temperature offset and scale

### Requirement: UNIT-002 Dimension safety
Conversions and arithmetic SHALL retain dimension metadata and SHALL reject incompatible operations with a stable error and source span.

#### Scenario: Incompatible conversion
- **WHEN** the user evaluates `1 kg -> m`
- **THEN** the system returns `E_UNIT_INCOMPATIBLE` and identifies both dimensions

### Requirement: UNIT-003 Versioned unit dataset
Every unit definition SHALL include a stable identifier, dimension, symbol, localized names and aliases, conversion rule, system or region where needed, and dataset version. Changes to conversion factors or aliases MUST include fixtures and provenance.

#### Scenario: Dataset migration
- **WHEN** the bundled unit dataset version changes
- **THEN** historical entries retain the original expression and result while new evaluations use the new version and record that version

### Requirement: UNIT-004 Ambiguity handling
The UI SHALL distinguish ambiguous names such as US and Imperial gallons and decimal and binary data units. It SHALL NOT silently choose a regional interpretation when multiple valid choices exist.

#### Scenario: Ambiguous gallon
- **WHEN** the user searches for `gallon` without a region
- **THEN** the UI offers `US gallon` and `Imperial gallon` with distinct symbols or labels before conversion

### Requirement: UNIT-005 Discoverable conversion UX
Users SHALL be able to write `value unit -> unit`, search units by symbol/name/alias, swap source and target, favorite pairs, and recall recent pairs.

#### Scenario: Unit alias search
- **WHEN** the user searches a localized alias for kilometers per hour
- **THEN** the canonical speed unit is returned without changing the persisted unit identifier

### Requirement: UNIT-006 No implicit live rates
Currency and cryptocurrency conversion SHALL NOT be part of the base offline capability. Any future live-rate feature MUST be a separate opt-in capability with source, timestamp, cache, stale-data, privacy, and network requirements.

#### Scenario: Currency request in base product
- **WHEN** the user requests a live currency conversion
- **THEN** the app explains that live rates are unavailable offline and does not synthesize or reuse an unlabeled stale rate

### Requirement: UNIT-007 Implicit popular data equivalents
When the user enters a data-size quantity without a target, the system SHALL keep that quantity as the primary value and present useful equivalents across decimal bits/bytes and binary bytes. It SHALL include only representations that remain readable at the entered scale and SHALL NOT silently reinterpret data size as data rate.

#### Scenario: Bare decimal megabytes
- **WHEN** the user evaluates `15 MB` or the Russian alias `15 мб`
- **THEN** the primary value is `15 MB`
- **AND** popular equivalents include `120 Mbit`, `0.015 GB`, approximately `14.305 MiB`, and `120000000 bit`

#### Scenario: Bare binary mebibytes
- **WHEN** the user evaluates `15 MiB` or `15 миб`
- **THEN** the primary value remains `15 MiB`
- **AND** decimal equivalents are derived from exactly `15728640 byte`, not from `15 MB`

### Requirement: UNIT-008 Unambiguous data aliases
Data aliases SHALL distinguish bits from bytes, decimal prefixes from binary prefixes, and data size from data rate. Case-insensitive Russian input SHALL map `мб` to decimal megabytes, `мбит` to decimal megabits, and `миб` to binary mebibytes. A `/s`, `/с`, `ps`, or rate word SHALL be required before a size is treated as a rate.

#### Scenario: Megabyte versus megabit
- **WHEN** the user evaluates `15 мб -> бит` and `15 мбит -> бит`
- **THEN** the exact results are `120000000 bit` and `15000000 bit` respectively

#### Scenario: Rate remains a rate
- **WHEN** the user enters `15 Мбит/с`
- **THEN** equivalents use data-rate units and do not include `MB` or `MiB` size values without a duration

### Requirement: UNIT-009 Category-driven converter workspace
The full workspace SHALL expose each supported offline dimension as a converter category. A converter SHALL provide source and target unit selection, value entry, swap, localized unit labels, and a result while reusing the same versioned unit dataset and evaluation semantics as expression conversion.

#### Scenario: Convert through controls
- **WHEN** the user selects Length, enters `10`, chooses kilometers as source and miles as target
- **THEN** the control-driven result equals evaluation of `10 km -> miles` under the same locale and dataset version

#### Scenario: Switch converter category
- **WHEN** the user changes from Length to Temperature
- **THEN** incompatible length units are replaced by temperature units and the interface preserves no invalid hidden conversion
