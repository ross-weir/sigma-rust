//! ErgoTree IR

// Coding conventions
#![forbid(unsafe_code)]
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(dead_code)]
#![deny(unused_imports)]
#![deny(missing_docs)]
// Clippy exclusions
#![allow(clippy::unit_arg)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(clippy::wildcard_enum_match_arm)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::todo)]
#![deny(clippy::unimplemented)]
#![deny(clippy::panic)]

pub mod chain;
pub mod constants;
mod utils;
pub mod wallet;

// Re-exported types from dependencies
pub extern crate ergo_chain_types;
pub extern crate ergo_merkle_tree;
pub extern crate ergo_nipopow;
/// Re-exported types from dependencies
#[cfg(feature = "rest")]
pub extern crate ergo_rest;
#[cfg(feature = "compiler")]
pub extern crate ergoscript_compiler;
pub extern crate ergotree_interpreter;
pub extern crate ergotree_ir;

/// Selectively exposed types
pub use utils::ArrLength;
