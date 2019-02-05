//! An example of drawing graphics on the watch display.

#![no_std]
#![no_main]

use mwatch_sdk::prelude::*;
use embedded_graphics::prelude::*;
use embedded_graphics::fonts::Font6x8;

use heapless::String;
use heapless::consts::*;
use core::fmt::Write;

static mut LAST: InputEvent = InputEvent::Multi;

#[no_mangle]
pub fn main() -> i32 {
    256
}

#[no_mangle]
pub fn update(system: &mut System) -> i32 {
    let mut string: String<U64> = String::new();
    write!(string, "Input: {:?}", unsafe { LAST }).unwrap();
    system.display.draw(Font6x8::render_str(string.as_str())
                    .translate(Coord::new(0, 16))
                    .with_stroke(Some(0xF818_u16.into()))
                    .into_iter()).unwrap();
    unsafe {
        LAST = InputEvent::Multi;
    }
    666
}

#[no_mangle]
pub fn input(_system: &mut System, input: InputEvent) -> i32 {
    unsafe { LAST = input }; 
    666
}