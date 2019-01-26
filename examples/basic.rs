//! An example of drawing graphics on the watch display.

#![no_std]
#![no_main]

extern crate mwatch_sdk;

#[no_mangle]
pub fn main() -> i32 {
    256
}

#[no_mangle]
pub fn update() -> i32 {
    666
}