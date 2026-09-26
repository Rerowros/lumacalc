## ADDED Requirements

### Requirement: GRAPH-001 Shared expression functions
The graphing workspace SHALL evaluate functions through the same expression AST, numeric context, variables, units policy, and error model as the calculator. The first release of graphing SHALL support explicit 2D Cartesian `y = f(x)` functions only.

#### Scenario: Calculator and graph parity
- **WHEN** a plotted function is sampled at an x-value also evaluated in the calculator
- **THEN** both paths produce equivalent values within the same numeric context

### Requirement: GRAPH-002 Bounded multi-function plot
The workspace SHALL plot up to five visible functions with distinct non-color-only identifiers, configurable range, show/hide controls, and auto-fit. Sampling MUST enforce point, recursion, precision, and elapsed-work limits.

#### Scenario: Discontinuity
- **WHEN** plotting a function with a discontinuity such as `1/x`
- **THEN** the renderer does not draw a misleading continuous segment across the discontinuity

### Requirement: GRAPH-003 Cancellable interaction
Pan, zoom, range edits, and function edits SHALL cancel or supersede obsolete sampling work. Interaction SHALL remain responsive while a lower-priority final-quality plot is produced.

#### Scenario: Rapid range changes
- **WHEN** the user repeatedly zooms before prior sampling completes
- **THEN** obsolete results cannot replace the most recent viewport and the UI remains within its interaction budget

### Requirement: GRAPH-004 Inspectable coordinates
The user SHALL be able to pan, zoom, reset, auto-fit, inspect cursor coordinates, and trace a selected function by keyboard or pointer.

#### Scenario: Keyboard trace
- **WHEN** focus is on a plotted function and the user invokes trace controls
- **THEN** the selected x and y values are exposed visually and through accessibility semantics

### Requirement: GRAPH-005 Accessible data alternative
Every plot SHALL provide a textual range summary and a keyboard-navigable table of representative sampled points. Color SHALL NOT be the only way to distinguish functions.

#### Scenario: Screen-reader alternative
- **WHEN** a screen-reader user opens the graph data view
- **THEN** function names, domain, range, discontinuities, and sampled coordinate pairs are available without interpreting the canvas

### Requirement: GRAPH-006 Deferred graph scope
3D, implicit, polar, parametric, symbolic derivative, equation solving, intersection finding, and image export SHALL require separate approved specifications.

#### Scenario: Unsupported plot type
- **WHEN** a user requests a deferred plot type
- **THEN** the workspace identifies the unsupported type and preserves the expression for editing
