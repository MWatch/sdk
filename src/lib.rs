#![no_std]

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};
use mabez_watch_sdk_core::{Table, TABLE_POINTER};

use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::PixelColorU16;
use embedded_graphics::drawable;

pub struct Display {}

#[link_section = ".entry_point"]
#[no_mangle]
/// The pointer the watch calls to start running this application.
pub static ENTRY_POINT: fn(*const Table) -> i32 = entry_point;

#[no_mangle]
/// The function called by the host to start us up. Does some setup, then
/// jumps to a function called `main` defined by the actual application using
/// this crate.
pub fn entry_point(raw_ctx: *const Table) -> i32 {
    // Turn the pointer into a reference and store in a static.
    unsafe {
        let ctx = &*raw_ctx;
        TABLE_POINTER = Some(ctx);
    };

    extern "C" {
        fn main() -> i32;
    }
    // call the user application
    unsafe { main() }
}

impl Display {
    pub fn draw<T>(&self, item_pixels: T) -> Result<(), ()>
    where
        T: Iterator<Item = Pixel<PixelColorU16>>,
    {
        let t = Table::get();
        let (width, height) = (128, 128);
        for drawable::Pixel(UnsignedCoord(x, y), color) in item_pixels {
            if x <= width && y <= height {
                (t.draw_pixel)(t.context, x as u8, y as u8, color.into_inner());
            }
        }
        Ok(())
    }
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}