---
description: "Create a new GitHub issue with standardized format, labels, and optional subtasks"
agent: pairing-companion
---

# Create Issue

Create a new GitHub issue on `mrampazz/koso`. Follow these steps.

## Step 1 — Understand what the human wants

The human's input: $ARGUMENTS

Ask clarifying questions if needed:
- What is the issue about?
- Is this a task (implementation work) or an epic (collection of tasks)?
- Which crate(s) does it affect? (for `crate:*` labels)
- What priority? (P0-critical, P1-high, P2-medium, P3-low)
- Does it belong to an existing epic? If so, which one?

## Step 2 — Draft the issue

Load and follow the `create-or-update-issue` skill to draft the issue body.

## Step 3 — Confirm with the human

Show the full issue (title, labels, body) to the human. Ask: *"Does this look right?"*

Make adjustments if needed.

## Step 4 — Create the issue

Create it:
```
gh issue create --repo mrampazz/koso --title "<title>" --label "<labels>" --body "<body>"
```

## Step 5 — Link to parent epic (if applicable)

If the issue belongs to an epic, add it as a sub-issue. Use the GitHub API:
```
gh api graphql -f query='mutation { addSubIssue(input: { issueId: "<epic-node-id>", subIssueId: "<new-issue-node-id>" }) { issue { id } } }'
```

To get node IDs:
```
gh api graphql -f query='{ repository(owner: "mrampazz", name: "koso") { issue(number: <N>) { id } } }'
```

## Step 6 — Report

Show the human the created issue URL and number.
