# Contributing to Minecraft Linker Bot

Thank you for your interest in contributing!

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- A Minecraft server for testing (optional but recommended)
- A Discord bot token for testing

## Workflow

This project follows a standard GitHub flow:

1. **Fork** the repository
2. **Create a branch** from `master` with a descriptive name:
   - `feat/your-feature-name` for new features
   - `fix/your-bug-name` for bug fixes

```bash
git checkout -b feat/your-feature-name
```

3. **Write your code**
4. **Commit** following the commit convention below
5. **Push** your branch
6. **Open a Pull Request** towards `master`

The CI will automatically run on your PR. It must pass before merging.

## Commit Convention

This project uses [Conventional Commits](https://www.conventionalcommits.org/):

feat: add death announcement for PvP
fix: villager death detected
chore: update dependencies
docs: update README


## Code Style

Before pushing, always run:

```bash
cargo fmt
cargo clippy -- -D warnings
```

The CI will reject any PR that does not pass these checks.

## Project Structure
```
src/
├── main.rs          # Entry point, startup and shutdown logic
├── stats/
│   ├── mod.rs
│   └── stats.rs     # Snapshot, stats parsing, end-of-session report
├── log/
│   ├── mod.rs
│   └── log.rs       # Log file watcher and event parsing
└── commands/
    ├── mod.rs
    └── commands.rs  # Discord slash commands
```
Feel free to add any modules if needed

## Opening an Issue

Before opening an issue please check if one already exists.

- **Bug** — describe what happened, what you expected, and how to reproduce it
- **Feature** — describe the feature and why it would be useful

## Questions

Feel free to open an issue with the `question` label.