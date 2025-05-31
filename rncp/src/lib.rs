#![no_std]
#![forbid(unstable_features)]
#![forbid(clippy::undocumented_unsafe_blocks)]

extern crate alloc;

// | Rewrite |
pub mod parser;

pub mod env;
pub mod error;
