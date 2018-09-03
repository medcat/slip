#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

extern crate regex;
extern crate serde;
extern crate term;

pub mod diag;
pub mod error;
pub mod stream;
pub mod syn;
pub mod visit;

fn main() {
    println!("Hello, world!");
}
