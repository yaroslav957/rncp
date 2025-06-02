mod arg;

use core::slice;

pub use crate::env::arg::*;

#[allow(dead_code)]
pub struct Env {
    args: &'static [Arg],
}
