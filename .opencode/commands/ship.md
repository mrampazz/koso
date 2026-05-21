---
description: "Commit, push, generate PR description, and open a pull request"
agent: pairing-companion
---

# Ship

Commit the current work, push, and open a PR. Follow these steps precisely.

## Pre-flight checks

Before doing anything:

1. Run `cargo check --workspace`. If it fails, stop and fix.
2. Run `cargo test --workspace`. If any tests fail, stop and fix.
3. Run `git status` to see what's staged/unstaged.
4. Run `git diff --stat` to see the scope of changes.

Show the human the results. Ask: *"Everything looks good to ship?"*

If the human says no, stop and address their concerns.

## Step 1 — Commit

If there are unstaged changes, stage them: `git add -A`

Check `git diff --cached --stat` to confirm what will be committed. Show the human.

Write a commit message following this format:
```
feat|fix|chore|refactor|test: <short description> (#<issue-number>)

<optional body explaining what and why>
```

The issue number should be in the commit title. Derive it from the branch name (e.g., `feat/4-pty-spawn` → `#4`).

Commit. Do NOT amend previous commits.

## Step 2 — Push

Push the branch:
```
git push -u origin <branch-name>
```

## Step 3 — Generate PR description

Load and follow the `pr-description` skill to generate the PR body.

## Step 4 — Create the PR

Create the PR using:
```
gh pr create --base main --head <branch> --title "<title>" --body "<generated body>"
```

The title should match the commit message format. Include `Closes #<number>` in the PR body.

## Step 5 — Link PR to issue

If the PR body includes `Closes #<number>`, GitHub will auto-link it. Verify by checking:
```
gh pr view <number> --json body
```

## Step 6 — Report

Show the human:
- PR URL
- What's included (files changed, tests added)
- The issue it closes

$ARGUMENTS
