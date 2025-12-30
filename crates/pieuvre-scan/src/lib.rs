pub mod engine;
pub mod error;
pub mod privileges;
pub mod remediation;

#[cfg(test)]
mod tests;

pub use error::ScanError;
pub type Result<T> = std::result::Result<T, ScanError>;
