//! An example of drawing graphics on the watch display.

#![no_std]
#![no_main]

extern crate mwatch_sdk;


use embedded_graphics::prelude::*;
use embedded_graphics::image::Image16BPP;
use mwatch_sdk::prelude::*;


#[no_mangle]
pub fn main() -> i32 {
    16
}

#[no_mangle]
pub fn update(system: &mut System) -> i32 {
    system.display.draw(Image16BPP::new(include_bytes!("./ff_nightly.raw"), 64, 64).translate(Coord::new(32,32)).into_iter()).unwrap();
    333
}

#[no_mangle]
pub fn input(_system: &mut System, _input: InputType) -> i32 {
    666
}