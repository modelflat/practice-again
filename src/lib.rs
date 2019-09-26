#![feature(test)]
extern crate test;

mod text_stats;
mod text_hiding;
mod xor;

pub mod utils;
pub use utils::OperationMode;

pub mod checksum;

pub use text_stats::*;
pub use text_hiding::*;
pub use xor::*;
