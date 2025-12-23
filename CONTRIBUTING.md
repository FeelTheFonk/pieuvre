# Contributing to pieuvre

Thank you for your interest in contributing to pieuvre. We welcome contributions from the community to help make Windows better for everyone.

## Getting Started

1. Fork the repository.
2. Clone your fork locally.
3. Create a feature branch:

```powershell
git checkout -b feature/your-feature-name
```

## Development Setup

### Prerequisites

- **Rust**: 1.75+
- **OS**: Windows 10/11 (Target platform)
- **Build Tools**: Visual Studio Build Tools (required for Windows API bindings)

### Build

```powershell
cargo build
```

### Test

We recommend using `nextest` for a better testing experience, but standard `cargo test` works as well.

```powershell
# Standard tests
cargo test

# Enhanced testing (recommended)
cargo nextest run
```

### Lint & Format

All code must be formatted and pass clippy checks before being merged.

```powershell
# Check for warnings
cargo clippy -- -D warnings

# Format code
cargo fmt
```

## Code Style

- **Conventions**: Follow standard Rust naming and structural conventions.
- **Documentation**: All public APIs must be documented using `///` comments.
- **Logging**: Use the `tracing` crate for all logging. Avoid `println!`.
- **Error Handling**: Use `anyhow::Result` in the CLI crate and `PieuvreError` in library crates.
- **Async**: Use `tokio` for all asynchronous operations and system monitoring.

## Commit Messages

We follow the Conventional Commits specification:

```text
feat: add MSI mode detection
fix: correct timer resolution on hybrid CPUs
docs: update installation instructions
refactor: simplify power plan switching
```

## Pull Request Process

1. Ensure all tests pass, including integration tests.
2. Update relevant documentation in the `docs/` directory.
3. Add a concise entry to `CHANGELOG.md`.
4. Request a review from the maintainers.

## Architecture Overview

- `pieuvre-common`: Shared types, error definitions, and utilities.
- `pieuvre-audit`: Read-only system inspection engine. **Must never modify system state.**
- `pieuvre-sync`: Core engine for system modifications and optimizations.
- `pieuvre-persist`: Snapshot management, compression (zstd), and rollback logic.
- `pieuvre-cli`: Command-line interface and TUI dashboard.

## Testing

### Unit Tests
Located within each module. Run with `cargo test`.

### Integration Tests
Integration tests require administrator privileges as they interact with the system.

```powershell
# Run as Administrator
cargo test --features integration
```

## Questions & Support

For questions, feature requests, or bug reports, please open a GitHub issue.
