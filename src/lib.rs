#![doc = include_str!("../doc/LIB.md")]
#![deny(unsafe_code, missing_docs, clippy::all, clippy::cargo)]

extern crate alloc;
extern crate core;

pub mod decimal;
pub use decimal::*;

pub mod int;
pub use int::*;

/// this struct is used for shadow the ParseError in decimal and int module.
/// for avoid ambiguous glob re-exports.
#[allow(hidden_glob_reexports, dead_code)]
struct ParseError;

pub use decimal::ParseError as DecimalParseError;
pub use int::ParseError as IntParseError;

mod utils;
mod config;

