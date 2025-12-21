//! Pieuvre - Types partagés et configuration
//!
//! Ce crate contient les types de données, erreurs, et configuration
//! partagés entre tous les modules Pieuvre.

mod config;
mod error;
mod types;

pub use config::*;
pub use error::*;
pub use types::*;
