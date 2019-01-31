//! An example of drawing graphics on the watch display.

#![no_std]
#![no_main]

use mwatch_sdk::prelude::*;


use embedded_graphics::prelude::*;
use embedded_graphics::fonts::Font6x8;
// use embedded_graphics::primitives::Circle;
// use embedded_graphics::primitives::Rect;
use heapless::String;
use heapless::consts::*;

use mwatch_sdk::System;

use core::fmt::Write;

static mut NUM: u32 = 0;

#[no_mangle]
pub fn main() -> i32 {
    // disp.draw(Circle::new(Coord::new(64,64), 30)
    //                 .with_stroke(Some(0x0523_u16.into()))
    //                 .with_fill(Some(0x0523_u16.into()))
    //                 .into_iter()).unwrap();
    // // uncomment this, rebuild and upload to the watch!
    // disp.draw(Rect::new(Coord::new(64 - 30 - 2, 64 - 30 - 2), Coord::new(64 + 30 + 2,64 + 30 + 2))
    //                 .with_stroke(Some(0xF818_u16.into()))
    //                 .with_stroke_width(4u8)
    //                 .into_iter()).unwrap();
    // disp.draw(Rect::new(Coord::new(0, 0), Coord::new(127, 127))
    //     .with_stroke(Some(0xF818_u16.into()))
    //     .with_fill(Some(0x2E5A_u16.into()))
    //     .with_stroke_width(4u8)
    //     .into_iter()).unwrap();
    // the bss region is not wiped!
    unsafe { NUM = 0; }
    0
}

#[no_mangle]
pub fn update(system: &mut System) -> i32 {
    
    unsafe { NUM += 1; }
    let mut string: String<U64> = String::new();
    write!(string, "Hello from Rust!").unwrap();
    system.display.draw(Font6x8::render_str(string.as_str())
                    .translate(Coord::new(0, 16))
                    .with_stroke(Some(0xF818_u16.into()))
                    .into_iter()).unwrap();
    string.clear();
    write!(string, "[{}]", unsafe { NUM }).unwrap();
    system.display.draw(Font6x8::render_str(string.as_str())
                    .translate(Coord::new(24, 48))
                    .with_stroke(Some(0xF818_u16.into()))
                    .into_iter()).unwrap();
    666
}

#[no_mangle]
pub fn input(_system: &mut System, _input: InputType) -> i32 {
    666
}