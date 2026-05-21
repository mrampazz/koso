---
description: "Split a GitHub issue into smaller subtask issues for incremental PRs"
agent: pairing-companion
---

# Split Issue

Break a GitHub issue into smaller, implementable subtask issues — each one small enough for a single PR.

## Step 1 — Fetch the issue

Run: `gh issue view $ARGUMENTS --repo mrampazz/koso --json number,title,body,labels,state`

Parse and display the issue to the human.

## Step 2 — Analyze the work

Read the issue body carefully. Identify:
- The overall objective
- The `## Tasks` checklist (if present)
- Acceptance criteria
- Which crates are affected (from `crate:*` labels)

## Step 3 — Propose subtasks

Load and follow the `split-issue-into-subtasks` skill.

Propose a breakdown where each subtask:
- Touches at most 2-3 files
- Is ~50-150 lines of changes
- Has a clear, testable definition of done
- Can be implemented and reviewed independently
- Has a logical ordering (dependencies flow top-to-bottom)

Show the proposed subtasks to the human:

```markdown
## Proposed split for #<N>: <title>

1. **<subtask title>** — <one-line description>
   - Crate: `koso-<x>`
   - Files: `<path1>`, `<path2>`
   - DoD: <what done looks like>

2. **<subtask title>** — <one-line description>
   ...
```

Ask: *"Does this split look right? Want to add, remove, or reorder anything?"*

## Step 4 — Create subtask issues

For each confirmed subtask, create a GitHub issue:
```
gh issue create --repo mrampazz/koso --title "<subtask title>" --label "<labels>" --body "<body>"
```

Each subtask body should include:
```markdown
## Objective

<description>

## Tasks

- [ ] <specific implementation step>
- [ ] <specific implementation step>

## Acceptance Criteria

- <testable criterion>

## Definition of Done

- [ ] Code compiles (`cargo check --workspace`)
- [ ] Tests pass (`cargo test -p <crate>`)
- [ ] New code has tests

Part of #<parent-issue-number>
```

Inherit the parent's `crate:*` and priority labels. Add any more specific labels if the subtask narrows the scope.

## Step 5 — Update the parent issue

Edit the parent issue body to reference the subtask issues:

```markdown
## Subtasks

- [ ] #<n1> <title>
- [ ] #<n2> <title>
- [ ] #<n3> <title>
```

Use `gh issue edit <parent-number> --repo mrampazz/koso --body "<updated body>"`.

## Step 6 — Report

Show the human all created subtask issue numbers and suggest starting with the first one.
