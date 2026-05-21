//! UI layout and widget rendering.
//!
//! Layout structure:
//! ┌──────────────────────────────────────────────┐
//! │ [status bar: git branch, cwd, session name]  │
//! ├──────────────────────────┬───────────────────┤
//! │                          │                   │
//! │   Main pane (blocks)     │  Side panel       │
//! │                          │  (scratch/AI)     │
//! │                          │                   │
//! ├──────────────────────────┴───────────────────┤
//! │ [input line: command entry + autocomplete]    │
//! └──────────────────────────────────────────────┘

// TODO: Implement layout rendering with ratatui
// TODO: Block widget (collapsible command + output)
// TODO: Status bar widget (git, cwd, session)
// TODO: Input line widget (with inline autocomplete ghost text)
// TODO: Side panel widget (scratch pad, AI chat)
// TODO: Pane split management (horizontal / vertical)
