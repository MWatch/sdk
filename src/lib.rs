#![no_std]

pub mod prelude;

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};
use mwatch_kernel_api::{CONTEXT_POINTER, CALLBACK_TABLE, Context, InputType};

use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::PixelColorU16;
use embedded_graphics::drawable;

#[link_section = ".entry_point"]
#[no_mangle]
/// The pointer the watch calls to start running this application.
pub static ENTRY_POINT: extern "C" fn() -> i32 = entry_point;

#[link_section = ".update_point"]
#[no_mangle]
/// The pointer the watch calls to start running this application.
pub static UPDATE_POINT: extern "C" fn(*mut Context<'static>) -> i32 = update_point;

#[link_section = ".input_point"]
#[no_mangle]
/// The pointer the watch calls to handle input
pub static INPUT_POINT: extern "C" fn(*mut Context<'static>, input: InputType) -> i32 = input_point;

extern "Rust" {
    fn main() -> i32;
    fn update(system: &mut System) -> i32; // TODO pass 'Resources setup in main to this update function'
    fn input(system: &mut System, input: InputType) -> i32;
}

#[no_mangle]
/// The function called by the host to start us up. Does some setup, then
/// jumps to a function called `main` defined by the actual application using
/// this crate.
pub extern "C" fn entry_point() -> i32 {
    unsafe { main() }
}

#[no_mangle]
/// Calls the user update function
pub extern "C" fn update_point(raw_ctx: *mut Context<'static>) -> i32 {
    // Turn the pointer into a reference and store in a static.
    unsafe {
        CONTEXT_POINTER = Some(&mut *raw_ctx);
    };

    unsafe { update(&mut SYSTEM) }
}

#[no_mangle]
/// Calls the user update function
pub extern "C" fn input_point(raw_ctx: *mut Context<'static>, state: InputType) -> i32 {
    // Turn the pointer into a reference and store in a static.
    unsafe {
        CONTEXT_POINTER = Some(&mut *raw_ctx);
    };

    // match input {
    //     InputType::Left(val) => {},
    //     InputType::Middle(val) => {},
    //     InputType::Right(val) => {},
    // }

    unsafe { input(&mut SYSTEM, state) }
}

static mut SYSTEM: System = System {
    display: Display { _0: 0u8 },
    logger: Logger { _0: 0u8 },
};

#[repr(C)]
pub struct System {
    pub display: Display,
    pub logger: Logger,
}

#[repr(C)]
pub struct Display {
    _0: u8
}

#[repr(C)]
pub struct Logger {
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

impl Logger {
    pub fn log_str(&mut self, args: &str) {
        use core::fmt::Write;
        self.write_str(args).unwrap();
    }

    /// Write a `format!`ed string to the itm port
    /// Using FMT bloats the binary by about 4K bytes, try to use log_str if possible
    pub fn log_fmt(&mut self, args: core::fmt::Arguments) {
        use core::fmt::Write;

        self.write_fmt(args).unwrap();
    }
}

impl core::fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let ctx = Context::get();
        (CALLBACK_TABLE.print)(ctx, s);
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