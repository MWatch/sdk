//! An example of drawing graphics on the watch display.

#![no_std]
#![no_main]

use mwatch_sdk::prelude::*;
use embedded_graphics::prelude::*;
use embedded_graphics::fonts::{Font6x8, Font6x12};

use heapless::String;
use heapless::consts::*;
use core::fmt::Write;

static mut LAST: InputEvent = InputEvent::Multi;

#[no_mangle]
pub fn main() -> i32 {
    256
}

#[no_mangle]
pub fn update(_system: &mut UserSpace, display: &mut Display) -> i32 {
    let mut string: String<U64> = String::new();
    display.draw(Font6x8::render_str("Last Input: ")
                    .translate(Coord::new(0, 8))
                    .with_stroke(Some(0xF818_u16.into()))
                    .into_iter()).unwrap();
    write!(string, "{:?}", unsafe { LAST }).unwrap();
    let input = Font6x12::render_str(string.as_str());
    display.draw(input
                    .translate(Coord::new(64 - input.size().0 as i32 / 2, 64 - input.size().1 as i32 / 2))
                    .with_stroke(Some(0xF818_u16.into()))
                    .into_iter()).unwrap();
    666
}

#[no_mangle]
pub fn input(system: &mut UserSpace, input: InputEvent) -> i32 {
    unsafe { LAST = input }; 
    system.logger.log_fmt(format_args!("INPUT: {:?}", input));
    666
}