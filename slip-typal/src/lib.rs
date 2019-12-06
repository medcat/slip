//! # Slip Typal
//! The intermediate representation for slip.  This is what most modules are
//! serialized to, especially when used as a plugin system.
#![warn(clippy::all)]
//#![warn(missing_docs)]

#[macro_use]
extern crate serde;
#[macro_use]
extern crate failure;

pub mod module;
mod runtime;
pub mod spec;
pub mod version;
