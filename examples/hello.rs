//! An example app showing that the app recieves processor time

#![no_std]
#![no_main]

use mwatch_sdk::prelude::*;
use embedded_graphics::prelude::*;
use embedded_graphics::fonts::Font6x8;
use heapless::String;
use heapless::consts::*;
use core::fmt::Write;

static mut NUM: u32 = 0;

#[no_mangle]
pub fn main() -> i32 {
    0
}

#[no_mangle]
pub fn update(system: &mut UserSpace) -> i32 {
    
    unsafe { NUM += 1; }
    let mut string: String<U64> = String::new();
    write!(string, "Hello from SDK!").unwrap();
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
pub fn input(_system: &mut UserSpace, _input: InputEvent) -> i32 {
    666
}