//! An examle app to show the panic logging feature to debug applications
//! 
//! 

#![no_std]
#![no_main]

use mwatch_sdk::prelude::*;

#[no_mangle]
pub fn main() -> i32 {
    panic!("Something deliberately went wrong!");
}

#[no_mangle]
pub fn update(_system: &mut UserSpace) -> i32 {
    666
}

#[no_mangle]
pub fn input(system: &mut UserSpace, input: InputEvent) -> i32 {
    system.logger.log_fmt(format_args!("Input recived: {:?}", input));
    666
}