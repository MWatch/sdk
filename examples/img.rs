//! An example of drawing graphics on the watch display.

#![no_std]
#![no_main]

extern crate mwatch_sdk;


use embedded_graphics::prelude::*;
use embedded_graphics::image::Image16BPP;
// use embedded_graphics::fonts::Font6x8;

use mwatch_sdk::Display;


#[no_mangle]
pub fn main() -> i32 {
    let disp = Display {}; //  TODO create this as a singleton
    disp.draw(Image16BPP::new(include_bytes!("./apple.raw"), 64, 64).translate(Coord::new(32,32)).into_iter()).unwrap();
    0
}

#[no_mangle]
pub fn update() -> i32 {
    666
}