# Skill: PR Description

Generate a pull request description from the current branch's changes.

---

## Process

### 1. Gather data

Run these commands and capture the output:

```bash
# Branch name
git branch --show-current

# Merge base
git merge-base HEAD main

# Commits on this branch
git log --oneline $(git merge-base HEAD main)..HEAD

# Full diff stats
git diff --stat $(git merge-base HEAD main)..HEAD

# Full diff (for analysis)
git diff $(git merge-base HEAD main)..HEAD
```

### 2. Analyze the changes

From the diff and commits, determine:

- **What changed** — list of modified/added/deleted files, grouped by crate
- **Why it changed** — derive from commit messages, issue references, and code context
- **What to test** — how a reviewer can verify the changes work
- **Risks** — anything that could break (API changes, dependency updates, unsafe code)

### 3. Extract issue number

From the branch name (e.g., `feat/4-pty-spawn` → `#4`), or from commit messages.

### 4. Generate the description

Use this format:

```markdown
## Summary

<1-2 sentence description of what this PR does and why.>

Closes #<issue-number>

## Changes

### `<crate-name>`
- <change description>
- <change description>

### `<other-crate>`
- <change description>

## How to Test

```sh
<commands to verify the changes>
```

## Notes for Reviewers

- <anything the reviewer should pay attention to>
- <design decisions that might not be obvious>
```

### 5. Conditional sections

Only include these if applicable:

- **Breaking Changes** — if public APIs changed
- **Dependencies Added** — if new crates were added to Cargo.toml
- **Related Issues** — if this PR relates to (but doesn't close) other issues

---

## Rules

1. **Be concise.** Reviewers skim PR descriptions. Every sentence should earn its place.
2. **Group changes by crate.** This is a workspace — reviewers need to know which crate is affected.
3. **Always include "How to Test"** with actual commands.
4. **Always include `Closes #N`** if the PR fully resolves an issue.
5. **Don't repeat the diff.** The reviewer can read the diff — explain the *why*, not the *what*.
