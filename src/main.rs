#![feature(never_type)]
#![feature(test)]
#![feature(map_get_key_value)]
#![warn(clippy::all)]
#![allow(dead_code)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate test;

pub mod diag;
pub mod error;
pub mod reduce;
pub mod stream;
pub mod syn;

fn main() {
    println!("Hello, world!");
}
