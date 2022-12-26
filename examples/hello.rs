//! An example app showing that the app recieves processor time

#![no_std]
#![no_main]

use core::fmt::Write;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::mono_font::ascii::FONT_6X12;
use embedded_graphics::pixelcolor::raw::RawU16;
use embedded_graphics::prelude::*;
use embedded_graphics::text::Alignment;
use embedded_graphics::text::Text;
use heapless::consts::*;
use heapless::String;
use mwatch_sdk::prelude::*;

static mut NUM: u32 = 0;

#[no_mangle]
pub fn main() -> i32 {
    0
}

#[no_mangle]
pub fn update(system: &mut UserSpace, display: &mut Display) -> i32 {
    writeln!(system.logger, "In update!").ok();
    let style = MonoTextStyle::new(&FONT_6X12, RawU16::from(0x02D4).into());
    let size = display.bounding_box().size;

    unsafe {
        NUM += 1;
    }
    let mut string: String<U64> = String::new();
    write!(string, "Hello from SDK!").unwrap();
    Text::with_alignment(
        &string,
        Point::new(size.width as i32 / 2, 20),
        style,
        Alignment::Center,
    )
    .draw(display)
    .ok();

    string.clear();
    write!(string, "[{}]", unsafe { NUM }).unwrap();
    Text::with_alignment(
        &string,
        Point::new(size.width as i32 / 2, 70),
        style,
        Alignment::Center,
    )
    .draw(display)
    .ok();

    writeln!(system.logger, "Update complete!!").ok();


    661
}

#[no_mangle]
pub fn input(system: &mut UserSpace, _input: InputEvent) -> i32 {
    writeln!(system.logger, "Hello from app!").ok();
    666
}
