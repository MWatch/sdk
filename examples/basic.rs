//! An example of drawing graphics on the watch display.

#![no_std]
#![no_main]

use mwatch_sdk::prelude::*;

#[no_mangle]
pub fn main() -> i32 {
    256
}

#[no_mangle]
pub fn update(system: &mut System) -> i32 {
    666
}