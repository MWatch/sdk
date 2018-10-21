//! An example of drawing graphics on the watch display.

#![no_std]
#![no_main]

extern crate mabez_watch_sdk;


use embedded_graphics::prelude::*;
use embedded_graphics::fonts::Font6x8;
use embedded_graphics::primitives::Circle;
// use embedded_graphics::primitives::Rect;

use mabez_watch_sdk::Display;


#[no_mangle]
pub fn main() -> i32 {
    let disp = Display {}; //  TODO create this as a singleton
    disp.draw(Font6x8::render_str("Hello from Rust!")
                    .translate(Coord::new(0, 16))
                    .with_stroke(Some(0xF818_u16.into()))
                    .into_iter()).unwrap();
    disp.draw(Circle::new(Coord::new(64,64), 30).with_stroke(Some(0xF818_u16.into())).into_iter()).unwrap();
    // uncomment this, rebuild and upload to the watch!
    // disp.draw(Rect::new(Coord::new(64 - 30, 64 - 30), Coord::new(64 + 30,64 + 30)).with_stroke(Some(0xF818_u16.into())).into_iter()).unwrap();
    0
}

#[no_mangle]
pub fn update() -> i32 {
    666
}