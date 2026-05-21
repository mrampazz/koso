# Skill: Split Issue into Subtasks

Decompose a GitHub issue into smaller, independently-implementable subtask issues.

---

## When to Use

Use this skill when an issue:
- Would require more than ~150 lines of changes
- Touches more than 3 files
- Has a `## Tasks` checklist with 4+ items
- Mixes concerns across multiple crates

## Splitting Principles

### 1. Each subtask = one reviewable PR

A subtask should be:
- **50-150 lines** of changes (not counting tests)
- **2-3 files** modified at most
- **Independently compilable** — `cargo check` passes after the subtask alone
- **Independently testable** — has its own tests or modifies existing ones
- **Self-contained** — a reviewer can understand it without reading other subtasks

### 2. Dependency ordering

Order subtasks so that:
- Types/traits come before implementations
- Core crate changes come before TUI crate changes
- Data models come before rendering
- Each subtask builds on the previous one's merged result

### 3. Stub pattern

If a later subtask depends on an API from an earlier one, the earlier subtask should include the full type signature (as a stub with `todo!()` if needed). This lets the later subtask fill in the implementation.

### 4. Common split patterns

| Original scope | Split into |
|---|---|
| New feature end-to-end | 1) Data types/model, 2) Core logic, 3) UI/rendering, 4) Tests |
| New trait + implementations | 1) Trait definition + first impl, 2) Additional impls |
| New widget | 1) Widget struct + basic render, 2) Interactivity/input handling, 3) Integration into layout |
| PTY feature | 1) PTY wrapper, 2) Async I/O bridge, 3) Integration with event bus |

---

## Subtask Issue Format

Each subtask issue should follow this template:

```markdown
## Objective

<What this subtask accomplishes. One paragraph max.>

## Tasks

- [ ] <Specific file and function to create/modify>
- [ ] <Specific file and function to create/modify>

## Acceptance Criteria

- <Testable criterion — what cargo test should verify>

## Definition of Done

- [ ] `cargo check --workspace` passes
- [ ] `cargo test -p <crate>` passes
- [ ] New public items have doc comments

Part of #<parent-issue-number>
```

**Labels:** Inherit the parent's priority label. Use the specific `crate:*` label(s) for this subtask (may be a subset of the parent's labels).

---

## Process

1. Read the parent issue thoroughly.
2. Identify natural boundaries (crate boundaries, type/impl boundaries, data/render boundaries).
3. Propose subtasks to the human with:
   - Title
   - One-line description
   - Crate affected
   - Estimated files touched
   - Dependencies on other subtasks
4. Get human confirmation before creating any issues.
5. Create subtask issues with `gh issue create`.
6. Update the parent issue body to list subtasks.

---

## Rules

1. **Never create subtasks without human confirmation.**
2. **Every subtask must compile independently** — no "part 1 doesn't compile, part 2 fixes it" splits.
3. **The sum of subtasks must equal the parent scope** — nothing lost, nothing added.
4. **Keep subtask count reasonable** — 3-6 subtasks per issue. If you'd need more than 6, the parent issue is probably an epic, not a task.
