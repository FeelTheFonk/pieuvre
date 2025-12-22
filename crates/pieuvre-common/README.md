# pieuvre-common

Shared types and error handling for pieuvre.

---

## Error Types

```rust
use pieuvre_common::PieuvreError;

pub enum PieuvreError {
    /// Windows API error
    WindowsApi(String),
    
    /// Registry operation failed
    Registry(String),
    
    /// Service control failed
    Service(String),
    
    /// File I/O error
    Io(std::io::Error),
    
    /// Configuration parsing error
    Config(String),
    
    /// Snapshot operation failed
    Snapshot(String),
    
    /// Permission denied
    PermissionDenied,
}
```

---

## Result Type

```rust
use pieuvre_common::Result;

pub type Result<T> = std::result::Result<T, PieuvreError>;
```

---

## Shared Structures

### ChangeRecord

```rust
pub struct ChangeRecord {
    pub change_type: ChangeType,
    pub name: String,
    pub original_value: String,
    pub new_value: String,
    pub timestamp: DateTime<Utc>,
}

pub enum ChangeType {
    Service,
    Registry,
    Firewall,
    Hosts,
    ScheduledTask,
}
```

### ServiceState

```rust
pub enum ServiceState {
    Running,
    Stopped,
    Disabled,
    Manual,
    Automatic,
}
```

---

## Usage

```rust
use pieuvre_common::{PieuvreError, Result, ChangeRecord};

fn example_operation() -> Result<()> {
    // Operation that might fail
    do_something().map_err(|e| PieuvreError::WindowsApi(e.to_string()))?;
    Ok(())
}
```
