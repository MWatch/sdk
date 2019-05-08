//! An example of using the kernel logging functionality

#![no_std]
#![no_main]

use mwatch_sdk::prelude::*;

#[no_mangle]
pub fn main() -> i32 {
    256
}

#[no_mangle]
pub fn update(system: &mut UserSpace, _display: &mut Display) -> i32 {
    system.logger.log_str("App updated");
    666
}

#[no_mangle]
pub fn input(system: &mut UserSpace, input: InputEvent) -> i32 {
    system.logger.log_fmt(format_args!("Input recived: {:?}", input));
    666
}