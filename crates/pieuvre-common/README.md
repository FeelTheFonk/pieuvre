# pieuvre-common

Shared types, error handling, and utilities for the pieuvre optimization toolkit.

---

## Overview

`pieuvre-common` provides the foundational building blocks used by all other crates in the workspace. It ensures consistency in error handling, configuration parsing, and data structures across the entire project.

## Key Components

### 1. Error Handling
Centralized error management using the `thiserror` crate.
- `PieuvreError`: The main error enum covering all possible failure modes (Registry, Service, IO, etc.).
- `Result<T>`: A type alias for `std::result::Result<T, PieuvreError>`.

### 2. Configuration
Shared configuration structures and parsing logic.
- `Config`: The root configuration object.
- `TelemetryConfig`: Specific settings for telemetry blocking.
- `PerformanceConfig`: Hardware-aware optimization parameters.

### 3. Shared Utilities
- **Logging**: Common tracing subscribers and formatting.
- **Validation**: Helper functions for system state validation.
- **Constants**: Global constants used across the workspace (e.g., registry paths, service names).

---

## API Usage

```rust
use pieuvre_common::{PieuvreError, Result};

fn example_function() -> Result<()> {
    // Shared error handling
    if something_failed() {
        return Err(PieuvreError::Internal("Something went wrong".into()));
    }
    Ok(())
}
```

---

## Dependency Graph

This crate has **zero dependencies** on other workspace crates to prevent circular dependencies. It is the root of the dependency tree.
