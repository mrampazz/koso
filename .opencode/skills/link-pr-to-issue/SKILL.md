# Skill: Link PR to Issue

Link a GitHub PR to an issue using the correct closing reference.

---

## Process

Since both PRs and issues live in the same repo (`mrampazz/koso`), use the simple format:

```
Closes #<issue-number>
```

This goes in the **PR body**, not the title.

### Steps

1. Determine the issue number from the branch name or commit messages.
   - Branch `feat/4-pty-spawn` → issue `#4`
   - Commit message `feat: spawn shell in PTY (#4)` → issue `#4`

2. Ensure the PR body contains `Closes #<N>` on its own line.

3. Verify the link was created:
   ```
   gh pr view <pr-number> --repo mrampazz/koso --json closingIssuesReferences
   ```

### Supported keywords

GitHub recognizes these keywords (case-insensitive):
- `Closes #N`
- `Fixes #N`
- `Resolves #N`

Use `Closes` as the default.

---

## Rules

1. **Always use `Closes`** unless the PR only partially addresses the issue (in which case, don't use a closing keyword — just reference with `Related to #N`).
2. **One PR should close one issue.** If a PR addresses multiple issues, each needs its own `Closes #N` line.
3. **Verify the link** after creating the PR.
