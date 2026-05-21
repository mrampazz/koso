# Config Loading & CLI Arg Parsing

## Objective

Make `Config::load()` robust and add `clap`-based CLI arg parsing so `koso` can be invoked with `--shell`, `--working-dir`, etc., with CLI args overriding config file values. Closes #7.

## Repo

`mrampazz/koso` — default repo for all tasks in this plan.

## Success Criteria

- `koso` starts with sensible defaults if no config file exists
- `koso --shell /bin/bash` overrides the shell
- `koso --working-dir /tmp` overrides the working directory
- `koso --config /path/to/config.toml` loads a custom config
- Invalid config prints a helpful error (not a panic)
- `cargo build` and `cargo test` pass clean

## Task List

- [x] Task 1: Add `clap` and `dirs` dependencies to workspace and relevant crates
- [x] Task 2: Define CLI struct with clap derive in `koso-tui` (flags: `--shell`, `--working-dir`, `--config`)
- [x] Task 3: Improve `Config::load()` — accept optional config path, use `dirs` crate for XDG, clear error on invalid TOML
- [x] Task 4: Add `Config::merge_cli()` method — CLI args override config file values
- [x] Task 5: Wire CLI parsing + config loading + merge together in `main.rs`
- [x] Task 6: Add unit tests for config (default, from file, CLI override, invalid TOML error)
- [x] Task 7: Verify `cargo build` and `cargo test` pass clean
