## ADDED Requirements

### Requirement: UI-001 Single editor-to-result flow
The main window SHALL present an editable expression, a visually dominant result, and immediate inline feedback without modal dialogs. The expression SHALL remain editable after both successful and failed evaluation.

#### Scenario: Evaluate and continue editing
- **WHEN** the user presses `Enter` on a valid expression
- **THEN** the result is committed to history, focus remains in the expression workflow, and the expression can be recalled or extended without pointer input

#### Scenario: Recover from an error
- **WHEN** evaluation fails
- **THEN** the error is shown adjacent to the relevant source span and correcting the text dismisses or updates it

### Requirement: UI-002 Adaptive workspace layout
The window SHALL adapt from a compact calculator to an expanded workspace. The on-screen keypad SHALL be hideable, and history or variables SHALL use a side panel at wide sizes and a dismissible drawer at narrow sizes.

#### Scenario: Compact resize
- **WHEN** the window is narrowed to the documented minimum width
- **THEN** the expression, result, core actions, and current context remain usable without horizontal scrolling

#### Scenario: Compact content density
- **WHEN** the quick overlay shows an error or up to four result/suggestion rows
- **THEN** its height follows the visible rows with only token spacing and no large reserved blank region above, between, or below them

#### Scenario: Expanded resize
- **WHEN** the window is widened
- **THEN** the app reveals additional workspace content without duplicating commands or losing editor focus

### Requirement: UI-003 Keyboard-first commands
Every calculation and navigation action SHALL have a keyboard path. Required bindings SHALL include `Enter` evaluate, `Esc` dismiss then clear, `Ctrl+L` clear input, `Ctrl+K` command palette, `Ctrl+F` history search, `Alt+Up/Down` history navigation, context-sensitive `Ctrl+C`, `Ctrl+Shift+C` full-precision result copy, and standard undo/redo. `Ctrl+C` SHALL copy the editor selection when one exists and otherwise copy the visible result.

#### Scenario: Complete calculation without a pointer
- **WHEN** the application starts and the user types `2+2`, presses `Enter`, and presses `Ctrl+C`
- **THEN** `4` is evaluated and copied without requiring a mouse or changing focus manually

#### Scenario: Shortcut discovery
- **WHEN** the user opens the command palette or a command tooltip
- **THEN** the active keyboard shortcut is displayed with the command name

### Requirement: UI-004 Safe paste and editor behavior
Pasted content SHALL be treated only as expression text, SHALL NOT execute shell commands, scripts, files, or network requests, and SHALL be constrained by parser limits. The editor SHALL provide selection, undo, redo, and plain-expression paste.

#### Scenario: Hostile pasted text
- **WHEN** pasted text resembles a shell command or URL
- **THEN** it is parsed only as calculator input and cannot invoke the operating system or network

### Requirement: UI-005 Visible mode context
The active workspace, angle unit, numeral base, word size, signedness, precision, and approximation state SHALL be visible whenever they affect the result. Changing context SHALL be undoable where practical and SHALL NOT mutate historical entries.

#### Scenario: Angle-mode visibility
- **WHEN** Scientific mode is active
- **THEN** `DEG` or `RAD` is visible without opening settings and can be changed from the keyboard

### Requirement: UI-006 Command palette
The command palette SHALL search modes, functions, conversions, settings, and actions by localized name, alias, and shortcut without requiring the user to memorize syntax.

#### Scenario: Discover unit conversion
- **WHEN** the user opens `Ctrl+K` and searches for `temperature`
- **THEN** the conversion workspace and relevant unit actions are offered and can be executed from the keyboard

### Requirement: UI-007 Theme and motion behavior
The interface SHALL support system light, dark, and high-contrast preferences, an explicit light/dark override, visible focus, and reduced motion. The default visual language SHALL be dark Fluent with original assets, a restrained blue-violet accent, flat navigation and result rows, and an opaque low-contrast surface with restrained shadow. The compact overlay SHALL keep a PowerToys Run-inspired hierarchy with a dominant expression field and secondary result text, while the full workspace SHALL use a Windows Calculator-inspired navigation hierarchy. Decorative animation MUST stop when idle and MUST NOT delay input. Decorative glow, traffic-light chrome, unrelated accents, blur, and redundant cards or capsules MUST NOT compete with the expression, result, or active mode.

#### Scenario: System theme change
- **WHEN** the OS theme or high-contrast preference changes while the app is running
- **THEN** the app updates its design tokens without losing input or requiring restart

#### Scenario: Compact and expanded visual consistency
- **WHEN** the same calculation is viewed in compact overlay and expanded workspace modes
- **THEN** both modes use the same semantic color, typography, spacing, radius, flat-row state, and exactness tokens while changing only information density and layout

#### Scenario: No redundant chrome or implementation metadata
- **WHEN** the full calculator workspace is visible
- **THEN** it does not repeat the application name or active mode in competing regions and does not expose engine implementation text such as precision-backend labels

### Requirement: UI-008 Clear destructive actions
Destructive actions such as clearing history or all local data SHALL require explicit intent and SHALL state their scope. History clearing SHALL offer a short undo window before irreversible deletion.

#### Scenario: Clear history with undo
- **WHEN** the user confirms clearing history and immediately invokes Undo
- **THEN** the cleared entries are restored without affecting settings or variables

### Requirement: UI-009 Global quick overlay
On Windows, the running application SHALL register `Win+Alt+Space` as a non-repeating system-wide hotkey. Invoking it SHALL show a compact always-on-top overlay, move keyboard focus to the expression editor, and permit immediate calculation or conversion. The overlay SHALL expose Expand, Hide, and Quit actions and SHALL report a hotkey conflict without disabling normal window use.

#### Scenario: Open hidden calculator
- **WHEN** the calculator process is running with its window hidden and the user presses `Win+Alt+Space`
- **THEN** a compact overlay appears on the active monitor with the expression editor focused

#### Scenario: Toggle visible overlay
- **WHEN** the quick overlay is already visible and the user presses `Win+Alt+Space` or `Esc`
- **THEN** the overlay hides while the event-driven hotkey listener remains available

#### Scenario: Hotkey conflict
- **WHEN** another process has already registered `Win+Alt+Space`
- **THEN** the calculator remains usable and shows a clear conflict status with an explicit retry action

### Requirement: UI-010 Overlay lifecycle and resource behavior
Overlay-ready background mode SHALL run inside the calculator process without a Windows service, polling timer, network request, or periodic repaint. Hiding the overlay SHALL retain only the application state and blocking hotkey listener; invoking Quit SHALL unregister the hotkey and terminate the process.

#### Scenario: Hide versus quit
- **WHEN** the user hides the overlay
- **THEN** the process remains available for the global hotkey with no periodic work
- **AND** when the user invokes Quit, no calculator process or registered hotkey remains

### Requirement: UI-011 Mode and converter navigation
Ordinary launch SHALL open the full workspace with discoverable navigation grouped into calculator modes, offline converter categories, and settings. Selecting Standard, Scientific, Programmer, Date calculation, or a supported converter SHALL replace the mode-specific work area without opening another window or losing unrelated history.

#### Scenario: Select a calculator mode
- **WHEN** the user opens navigation and selects Scientific
- **THEN** Scientific is visibly selected and the work area exposes scientific controls with the shared expression and result model

#### Scenario: Select a converter category
- **WHEN** the user selects Length under converters
- **THEN** the work area shows source and target unit controls, swap, value entry, and a converted result without requiring expression syntax
