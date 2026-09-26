## ADDED Requirements

### Requirement: STATE-001 Context-complete history
Each successful history entry SHALL retain the expression, canonical value, formatted result, exactness, mode context, referenced variable snapshot, unit dataset version, timestamp, and app schema version needed to explain or reproduce it.

#### Scenario: Historical context remains stable
- **WHEN** precision, angle mode, or a referenced variable later changes
- **THEN** the stored history row continues to display the original result and original context

### Requirement: STATE-002 Searchable and bounded history
History SHALL support search, pinning, copy, edit-and-rerun, grouping by session or day, and configurable age/count retention. Searching 10,000 representative entries MUST satisfy the performance budget.

#### Scenario: Search result reuse
- **WHEN** a user finds an old expression and chooses edit-and-rerun
- **THEN** the expression is loaded into the editor without overwriting the historical entry

### Requirement: STATE-003 Explicit replay semantics
When a historical expression references variables, replay SHALL let the user choose the saved variable snapshot or current values whenever the results can differ.

#### Scenario: Replay after variable change
- **WHEN** `tax` changed after a saved `price * (1 + tax)` calculation
- **THEN** replay offers saved-context reproduction and current-context recalculation with clear labels

### Requirement: STATE-004 Named variables
The MVP SHALL support identifier assignment such as `tax = 0.2`, read-only `Ans`, listing, editing, copy, and delete. Reassignment SHALL be explicit, and cyclic definitions SHALL be rejected.

#### Scenario: Variable assignment and reuse
- **WHEN** the user evaluates `tax = 0.2` and then `100 * (1 + tax)`
- **THEN** the second result is `120` and its history stores the `tax` snapshot

#### Scenario: Cyclic variable
- **WHEN** a definition directly or indirectly references itself
- **THEN** it is rejected with `E_VARIABLE_CYCLE` and prior valid values remain unchanged

### Requirement: STATE-005 Memory registers
The workspace SHALL expose `M+`, `M-`, `MR`, and `MC` with a visible memory-state indicator. Memory changes SHALL be local and deterministic across modes.

#### Scenario: Add to memory
- **WHEN** memory is empty, the current value is `10`, and the user invokes `M+`
- **THEN** memory becomes `10` and `MR` inserts or recalls that retained value

### Requirement: STATE-006 Private session
A private session SHALL calculate normally without persisting history, variables, searches, or recent conversion pairs after the session ends.

#### Scenario: Exit private session
- **WHEN** the user closes a private session
- **THEN** no entries or variables created in that session appear after restart

### Requirement: STATE-007 Non-blocking persistence
History and setting writes MUST NOT block the UI thread. A failed persistence operation SHALL leave the evaluated result usable and SHALL report a recoverable local-storage error.

#### Scenario: Storage temporarily unavailable
- **WHEN** an expression evaluates successfully but its history write fails
- **THEN** the result remains visible, the app reports that it was not saved, and it does not freeze or crash
