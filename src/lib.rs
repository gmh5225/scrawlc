//! # `scrawlc`
//!
//! Library of Scarwl's compiler.

mod analyzer;
mod tests;

pub use analyzer::{Position, Scanner, ScannerError, Token, ETX, LF};
