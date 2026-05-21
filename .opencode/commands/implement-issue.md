---
description: "Implement a GitHub issue: fetch it, create a branch, split into subtasks if needed, and start the pairing workflow"
agent: pairing-companion
---

# Implement Issue

You are starting work on a GitHub issue. Follow these steps precisely.

## Step 1 — Fetch the issue

Run: `gh issue view $ARGUMENTS --repo mrampazz/koso --json number,title,body,labels,state`

Parse the response. Show the human:
- Issue number and title
- Labels
- The full body (tasks, acceptance criteria, etc.)

If the issue is closed, warn the human and ask if they want to reopen it.

## Step 2 — Check for subtasks

Read the issue body. If it has a `## Tasks` section with a checklist, those are the subtasks.

If the issue has **no subtasks** and the body describes work that would touch more than ~3 files or ~150 lines of code, **stop and suggest splitting**. Tell the human:

> "This issue looks like it could be broken into smaller subtasks for easier review. Want me to split it using `/split-issue`?"

If the human agrees, load and follow the `split-issue-into-subtasks` skill.

## Step 3 — Determine affected crates

Look at the issue labels for `crate:*` labels (e.g., `crate:core`, `crate:mux`). These tell you which crates are affected.

If no crate labels exist, read the issue body and infer which crates are involved. Confirm with the human.

## Step 4 — Create a feature branch

Naming convention: `feat/<issue-number>-<short-description>`

Examples:
- Issue #4 "Spawn shell in PTY" → `feat/4-pty-spawn`
- Issue #13 "Block data model" → `feat/13-block-data-model`

Ensure you're on `main` and it's up to date before branching:
```
git checkout main
git pull origin main
git checkout -b feat/<number>-<slug>
```

## Step 5 — Move issue to In Progress (if project board is configured)

Check `.opencode/config.json`. If `projectBoard.id` is non-empty and `statusOptions.inProgress` is set:

```
gh api graphql -f query='mutation { updateProjectV2ItemFieldValue(input: { projectId: "<projectBoard.id>", itemId: "<item-id>", fieldId: "<statusFieldId>", value: { singleSelectOptionId: "<statusOptions.inProgress>" } }) { projectV2Item { id } } }'
```

If the project board is not configured, skip this step silently.

## Step 6 — Start the pairing workflow

Now proceed with the standard pairing-companion Phase 1 (Understand the Objective) using the issue body as the starting context. The objective is already defined by the issue — confirm it with the human and create the plan.

**Important:** Each task in the plan should map to a single commit. The plan name should match the branch slug (e.g., `pty-spawn` for branch `feat/4-pty-spawn`).

## Step 7 — Post-implementation checks

After all tasks are complete:

1. Run `cargo check --workspace`
2. Run `cargo test --workspace`
3. Run `cargo clippy --workspace -- -D warnings` (if available)
4. If any fail, surface and fix before considering the work done.
