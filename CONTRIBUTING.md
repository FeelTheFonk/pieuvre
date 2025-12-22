# Contributing to pieuvre

Thank you for your interest in contributing.

## Getting Started

1. Fork the repository
2. Clone your fork locally
3. Create a feature branch

```powershell
git checkout -b feature/your-feature-name
```

## Development Setup

### Prerequisites

- Rust 1.75+
- Windows 10/11 (target platform)
- Visual Studio Build Tools (for Windows API bindings)

### Build

```powershell
cargo build
```

### Test

```powershell
# Standard tests
cargo test

# SOTA tests (recommended)
cargo nextest run
```

### Lint

```powershell
cargo clippy -- -D warnings
```

### Format

```powershell
cargo fmt
```

## Code Style

- Follow Rust standard conventions
- Document public APIs with `///` comments
- Use `tracing` for logging (not `println!`)
- Handle errors with `anyhow::Result` in CLI, `PieuvreError` in libraries
- **Zero Clippy Warnings**: All PRs must pass `cargo clippy` without warnings.
- **Async First**: Use `tokio` for any I/O or monitoring tasks.

## Commit Messages

Use conventional commits:

```
feat: add MSI mode detection
fix: correct timer resolution on hybrid CPUs
docs: update installation instructions
refactor: simplify power plan switching
```

## Pull Request Process

1. Ensure all tests pass
2. Update documentation if needed
3. Add entry to CHANGELOG if applicable
4. Request review from maintainers

## Architecture

- `pieuvre-common`: Shared types and errors
- `pieuvre-audit`: Read-only system inspection
- `pieuvre-sync`: System modification functions
- `pieuvre-persist`: Snapshot and rollback
- `pieuvre-cli`: Command-line interface

Changes should respect this separation. Audit functions must not modify state.

## Testing

### Unit Tests

Located in each module alongside the code.

### Integration Tests

Run with administrator privileges:

```powershell
# Run as Administrator
cargo test --features integration
```

## Questions

Open an issue for questions or discussions.
