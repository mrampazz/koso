---
description: "Show the status of an epic's sub-issues, grouped by state"
agent: pairing-companion
---

# Epic Status

Show the current status of an epic and its sub-issues.

## Step 1 — Determine the epic

The human's input: $ARGUMENTS

If a number is provided, use it directly. If not, list all open epics:
```
gh issue list --repo mrampazz/koso --label epic --state open --json number,title
```

Show the list and ask the human to pick one.

## Step 2 — Fetch the epic and its sub-issues

Fetch the epic:
```
gh issue view <number> --repo mrampazz/koso --json number,title,body,labels,state
```

List all issues that reference this epic. Search for issues mentioning "Part of Epic #<number>":
```
gh issue list --repo mrampazz/koso --search "Part of Epic #<number>" --state all --json number,title,state,labels --limit 50
```

Also check the epic body for acceptance criteria checkboxes.

## Step 3 — Group and display

Group sub-issues into:

### Closed (recently)
Issues closed in the last 7 days. Show with checkmark.

### Has PR
Open issues with a linked PR. Show PR number and status:
```
gh pr list --repo mrampazz/koso --search "head:<branch>" --json number,title,state
```

### Open
Remaining open issues. Show priority label.

### Format

```markdown
## Epic #<N>: <title>

**Progress:** X/Y tasks complete

### Done
- [x] #<n> <title>

### In Progress (has PR)
- [ ] #<n> <title> — PR #<pr> (draft|open|approved)

### Open
- [ ] #<n> <title> [P0-critical]
- [ ] #<n> <title> [P1-high]
```

## Step 4 — Offer next action

Ask: *"Want to start working on one of the open issues?"*

If yes, suggest the highest-priority open issue and offer to run `/implement-issue <number>`.
