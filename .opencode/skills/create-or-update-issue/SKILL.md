# Skill: Create or Update Issue

Create or update a GitHub issue on `mrampazz/koso` with a standardized format.

---

## Issue Types

### Epic

Epics are top-level feature areas that contain multiple task issues.

**Labels:** Always include `epic` plus a priority label.

**Template:**
```markdown
## Overview

<What this epic covers and why it matters.>

## Acceptance Criteria

- [ ] <Testable criterion 1>
- [ ] <Testable criterion 2>

## Subtasks

_See linked issues below_
```

### Task

Tasks are individual implementation issues, small enough for 1-3 PRs.

**Labels:** Always include a priority label (`P0-critical`, `P1-high`, `P2-medium`, `P3-low`) and one or more `crate:*` labels.

**Template:**
```markdown
## Objective

<What needs to be done and why.>

## Tasks

- [ ] <Specific implementation step>
- [ ] <Specific implementation step>

## Acceptance Criteria

- <Testable criterion>
- <Testable criterion>

## Definition of Done

- [ ] Code compiles (`cargo check --workspace`)
- [ ] Tests pass (`cargo test --workspace`)
- [ ] New public APIs have doc comments
- [ ] New code has unit tests

Part of Epic #<epic-number>
```

---

## Labels

Available labels and when to use them:

| Label | When |
|---|---|
| `epic` | Top-level feature area |
| `P0-critical` | Must have for MVP |
| `P1-high` | Important, needed soon after MVP |
| `P2-medium` | Nice to have |
| `P3-low` | Future / stretch goal |
| `crate:core` | Changes to koso-core |
| `crate:tui` | Changes to koso-tui |
| `crate:mux` | Changes to koso-mux |
| `crate:ai` | Changes to koso-ai |
| `crate:complete` | Changes to koso-complete |
| `crate:plugin` | Changes to koso-plugin |

---

## Rules

1. **Always include a priority label** on every issue.
2. **Always include `crate:*` labels** on task issues.
3. **Always include "Part of Epic #N"** on task issues that belong to an epic.
4. **Acceptance criteria must be testable** — not vague ("works well") but specific ("returns exit code 0").
5. **When updating an existing issue**, preserve all existing content that is still accurate. Only modify what needs changing.
6. **Task lists use `- [ ]` checkboxes** for trackable progress.
