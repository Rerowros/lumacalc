## ADDED Requirements

### Requirement: MODE-001 Scientific function set
Scientific mode SHALL provide trigonometric and inverse trigonometric functions, hyperbolic functions, natural and base-10 logarithms, arbitrary-base logarithm, exponentials, powers, roots, absolute value, factorial, permutations, combinations, and constants `pi`, `e`, and `tau`.

#### Scenario: Degree calculation
- **WHEN** angle context is `DEG` and the user evaluates `sin(30)`
- **THEN** the result is mathematically equivalent to `0.5` within the configured precision

#### Scenario: Explicit angle unit
- **WHEN** the user evaluates `sin(0.5 rad)` while the global context is `DEG`
- **THEN** the explicit `rad` annotation takes precedence for that argument

### Requirement: MODE-002 Scientific notation controls
The user SHALL be able to choose significant digits and standard, scientific, or engineering notation without changing the retained numeric value.

#### Scenario: Formatting-only change
- **WHEN** a result is switched from standard to engineering notation
- **THEN** only its representation changes and subsequent calculations continue from the original retained value

### Requirement: MODE-003 Programmer integer context
Programmer mode SHALL support binary, octal, decimal, and hexadecimal literals and display; word sizes 8, 16, 32, and 64; signed and unsigned interpretation; bitwise `AND`, `OR`, `XOR`, `NOT`; shifts; and rotates.

#### Scenario: Multi-base representation
- **WHEN** the current value is hexadecimal `0xFF` with an 8-bit word size
- **THEN** synchronized BIN, OCT, DEC, and HEX representations are visible and signed interpretation is `-1` while unsigned interpretation is `255`

### Requirement: MODE-004 Explicit overflow and truncation
Programmer operations SHALL apply the selected word-size policy deterministically and SHALL surface overflow, sign change, or truncation rather than silently discarding information.

#### Scenario: Eight-bit overflow
- **WHEN** unsigned 8-bit context evaluates `255 + 1`
- **THEN** the configured wrap or checked policy is applied and the UI visibly reports the overflow event

### Requirement: MODE-005 Interactive bit field
Programmer mode SHALL expose every bit in the active word, allow pointer and keyboard toggling, and update all base representations atomically.

#### Scenario: Toggle a bit
- **WHEN** the user toggles bit 3 of zero in an 8-bit word
- **THEN** the value becomes binary `00001000`, decimal `8`, and hexadecimal `08`

### Requirement: MODE-006 Shared engine semantics
Standard, Scientific, and Programmer workspaces SHALL use the same lexer, parser, AST, error model, history contract, and variable system. UI controls MUST emit expression operations rather than implement arithmetic independently.

#### Scenario: Typed and button-entered parity
- **WHEN** an expression is entered once by keyboard and once using on-screen controls
- **THEN** both paths produce identical expression tokens, result metadata, and history entries

### Requirement: MODE-007 Deferred advanced mathematics
Complex numbers, matrices, statistics, symbolic algebra, equation solving, and user-defined functions SHALL remain disabled until separate capability specifications are approved.

#### Scenario: Unsupported complex input
- **WHEN** a user requests a complex-only result in the MVP
- **THEN** the app returns a specific unsupported-capability message rather than inventing a partial implementation
