## ADDED Requirements

### Requirement: A11Y-001 Complete keyboard operation
All interactive controls, panels, dialogs, plot alternatives, and commands MUST be reachable and operable by keyboard with a deterministic focus order, visible focus indicator, and no keyboard traps.

#### Scenario: Keyboard-only primary journey
- **WHEN** a user launches the app and uses only the keyboard to calculate, inspect history, change angle mode, copy full precision, and return to the editor
- **THEN** every step is available with visible focus and no inaccessible intermediate state

### Requirement: A11Y-002 Windows accessibility semantics
Interactive and informative UI elements SHALL expose appropriate role, name, value, state, relationships, and actions through Windows accessibility APIs. Custom controls MUST meet the same contract as standard controls.

#### Scenario: Result announcement
- **WHEN** the user commits an evaluation
- **THEN** the screen reader announces the result or error once in a meaningful order and does not announce a changing preview on every keystroke

### Requirement: A11Y-003 Scaling and reflow
The UI SHALL remain usable at 100%, 150%, and 200% display scaling and at the minimum supported window size. Primary actions MUST NOT be clipped or require horizontal scrolling.

#### Scenario: Two-hundred-percent scaling
- **WHEN** the app runs at 200% scaling in each MVP workspace
- **THEN** expression, result, context, primary controls, and error messages remain reachable and readable

### Requirement: A11Y-004 Contrast and non-color cues
Normal text SHALL meet at least 4.5:1 contrast, large text and essential control boundaries at least 3:1, and state/error/function distinctions SHALL include a cue other than color. High-contrast mode SHALL replace decorative styling with system-readable tokens.

#### Scenario: High-contrast programmer view
- **WHEN** high-contrast mode is active
- **THEN** selected base, toggled bits, warnings, and focus are distinguishable without relying on original theme colors

### Requirement: A11Y-005 Motion and target sizing
The UI SHALL honor reduced-motion preferences, avoid flashing, and use pointer targets suitable for touch in standard layouts while allowing an explicit compact density.

#### Scenario: Reduced motion
- **WHEN** reduced motion is enabled
- **THEN** panel and result transitions become immediate or minimal without removing state feedback

### Requirement: A11Y-006 Locale-aware expression input
Display formatting, unit names, messages, dates, and grouping SHALL follow the selected locale. The expression editor SHALL accept the locale decimal separator while retaining a canonical invariant representation internally and preventing ambiguous tokenization. In decimal-comma locales, semicolon SHALL delimit function arguments so a comma is never guessed as both decimal and argument separator.

#### Scenario: Decimal comma input
- **WHEN** a comma-decimal locale user enters `1,5 + 2,5`
- **THEN** the expression evaluates to the locale-formatted value `4` and is stored canonically without changing its mathematical meaning

#### Scenario: Decimal-comma function arguments
- **WHEN** a comma-decimal locale user enters `log(2; 8)`
- **THEN** the parser interprets two arguments and returns `3` without treating either comma role ambiguously

### Requirement: A11Y-007 Initial language support
The first release SHALL ship complete English and Russian UI/message catalogs and SHALL fall back to English keys only in development builds. Missing production translations MUST fail a localization completeness gate.

#### Scenario: Russian error message
- **WHEN** Russian is selected and an expression has a division-by-zero error
- **THEN** the user sees a Russian message derived from the stable error code, not an untranslated internal string
