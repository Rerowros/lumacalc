## ADDED Requirements

### Requirement: EXPR-001 Stable expression grammar
The system SHALL parse a documented expression grammar with decimal and prefixed integer literals, parentheses, unary operators, binary operators, postfix operators, function calls, unit annotations, conversions, assignments, and constants. Multiplication and division SHALL bind before addition and subtraction, exponentiation SHALL be right-associative, and every grammar revision SHALL be versioned with golden fixtures.

#### Scenario: Operator precedence
- **WHEN** the user evaluates `2 + 3 * 4` and `(2 + 3) * 4`
- **THEN** the results are `14` and `20` respectively

#### Scenario: Right-associative exponentiation
- **WHEN** the user evaluates `2 ^ 3 ^ 2`
- **THEN** the expression is interpreted as `2 ^ (3 ^ 2)` and returns `512`

#### Scenario: Optional calculator-style equals prefix
- **WHEN** the user enters `=5+5` in the quick overlay
- **THEN** one leading equals sign is treated as an evaluation prefix and the result is `10`

### Requirement: EXPR-002 Explicit numeric context
The evaluator SHALL use an explicit immutable context containing precision, rounding mode, angle unit, integer word size, signedness, and numeral base. A calculation result SHALL retain the context used to produce it.

#### Scenario: Context does not rewrite prior work
- **WHEN** a result is calculated in degree mode and the current mode later changes to radians
- **THEN** the stored result retains degree context and is not silently recalculated

### Requirement: EXPR-003 Exactness and visible approximation
The evaluator SHALL preserve exact integers and SHALL track whether a real result was rounded or approximated. The UI SHALL render exact results with `=` and inexact results with `≈`; it SHALL NOT expose raw internal `NaN`, infinity, or panic text as a normal result.

#### Scenario: Rounded real result
- **WHEN** the selected display precision cannot represent the full result of `1 / 3`
- **THEN** the result is prefixed with `≈` and full-precision copy remains available within the configured computation precision

#### Scenario: Exact integer result
- **WHEN** the user evaluates `50!` within the configured safety limit
- **THEN** the complete exact integer is retained even if the display uses grouped or scientific formatting

### Requirement: EXPR-004 Stable structured errors
Every rejected expression SHALL return a stable error code, localized message key, source span, and optional recovery hints. Required error families SHALL include syntax, unknown symbol, division by zero, domain, incompatible dimensions, overflow/truncation, cyclic definition, and computation limit.

#### Scenario: Syntax error span
- **WHEN** the user evaluates `2 + * 3`
- **THEN** the system returns `E_PARSE_UNEXPECTED_TOKEN` with a source span covering the unexpected `*` and leaves the expression editable

#### Scenario: Domain error
- **WHEN** the user evaluates `sqrt(-1)` while complex numbers are disabled
- **THEN** the system returns `E_DOMAIN` with a human-readable constraint instead of a raw `NaN`

### Requirement: EXPR-005 Bounded evaluation
The engine MUST enforce configurable limits for source length, token count, AST depth, numeric precision, exponent range, factorial input, recursion, output digits, and elapsed work. Work that can exceed the UI-thread budget MUST be cancellable.

#### Scenario: Excessive computation
- **WHEN** an expression exceeds a configured computation limit
- **THEN** evaluation stops deterministically with `E_LIMIT_EXCEEDED`, the UI remains responsive, and no partial value is persisted as a successful result

#### Scenario: Superseded evaluation
- **WHEN** a long-running preview is in progress and the input changes
- **THEN** the obsolete work is cancelled or ignored and cannot overwrite the newer preview

### Requirement: EXPR-006 Deterministic formatting and copy
The formatter SHALL support standard, scientific, engineering, and programmer representations; significant-digit and grouping controls; locale-aware display; and an invariant full-precision copy representation.

#### Scenario: Display and invariant copy
- **WHEN** the locale displays decimal comma and digit grouping
- **THEN** the visible result follows the locale while the explicit full-precision copy command produces a stable locale-independent representation

### Requirement: EXPR-007 Headless contract
All parsing, evaluation, conversion, formatting, and error behavior SHALL be usable without creating a GUI window, and the same fixture corpus MUST drive headless tests and UI integration tests.

#### Scenario: Fixture parity
- **WHEN** a golden expression fixture is executed through the headless harness and through the desktop adapter
- **THEN** both paths produce the same value, exactness, context, and error metadata

### Requirement: EXPR-008 Calculator percent semantics
A postfix percent SHALL divide its operand by 100. When a percent expression is the direct right operand of addition or subtraction, it SHALL be relative to the left operand; multiplication and division SHALL use the ordinary fractional value. Parentheses SHALL allow the user to request ordinary arithmetic explicitly.

#### Scenario: Standalone percent
- **WHEN** the user evaluates `10%`
- **THEN** the exact result is `0.1`

#### Scenario: Relative addition
- **WHEN** the user evaluates `200 + 10%`
- **THEN** the result is `220`

#### Scenario: Relative fifty percent addition
- **WHEN** the user evaluates `100 + 50%`
- **THEN** the result is `150`, not `100.5`

#### Scenario: Multiplicative percent
- **WHEN** the user evaluates `200 * 10%`
- **THEN** the result is `20`

### Requirement: EXPR-009 Repeat last binary operation
After a committed Standard calculation whose final operation is an eligible binary operation, evaluating again with no new input SHALL apply that operation and its right operand to the previous result using the original numeric context. The repeated action SHALL be visible in history and clearable by editing or clearing the expression.

#### Scenario: Repeat addition
- **WHEN** the user evaluates `2 + 3` and then invokes evaluate again without editing
- **THEN** the next result is `8` and history identifies the repeated `+ 3` operation
