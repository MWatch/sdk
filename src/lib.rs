#![no_std]

pub mod prelude;

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};
use mwatch_kernel_api::{CONTEXT_POINTER, CALLBACK_TABLE, Context};

use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::PixelColorU16;
use embedded_graphics::drawable;

#[link_section = ".entry_point"]
#[no_mangle]
/// The pointer the watch calls to start running this application.
pub static ENTRY_POINT: fn() -> i32 = entry_point;

#[link_section = ".update_point"]
#[no_mangle]
/// The pointer the watch calls to start running this application.
pub static UPDATE_POINT: fn(*mut Context<'static>) -> i32 = update_point;

extern "Rust" {
    fn main() -> i32;
    fn update(system: &mut System) -> i32; // TODO pass 'Resources setup in main to this update function'
}

#[no_mangle]
/// The function called by the host to start us up. Does some setup, then
/// jumps to a function called `main` defined by the actual application using
/// this crate.
pub fn entry_point() -> i32 {
    unsafe { main() }
}

#[no_mangle]
/// Calls the user update function
pub fn update_point(raw_ctx: *mut Context<'static>) -> i32 {
    // Turn the pointer into a reference and store in a static.
    unsafe {
        CONTEXT_POINTER = Some(&mut *raw_ctx);
    };

    unsafe { update(&mut SYSTEM) }
}

static mut SYSTEM: System = System {
    display: Display { _0: 0u8 }
};

#[repr(C)]
pub struct System {
    pub display: Display,
}

#[repr(C)]
pub struct Display {
    _0: u8
}


impl Display {
    pub fn draw<T>(&self, item_pixels: T) -> Result<(), ()>
    where
        T: Iterator<Item = Pixel<PixelColorU16>>,
    {
        let ctx = Context::get();
        let (width, height) = (128, 128);
        for drawable::Pixel(UnsignedCoord(x, y), color) in item_pixels {
            if x <= width && y <= height {
                (CALLBACK_TABLE.draw_pixel)(ctx, x as u8, y as u8, color.into_inner());
            }
        }
        Ok(())
    }
}

// TODO notify the kernel this app has crashed?
#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}