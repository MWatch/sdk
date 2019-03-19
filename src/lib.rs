#![no_std]

pub mod prelude;
mod display;
mod logger;

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};
use mwatch_kernel_api::{CONTEXT_POINTER, Context, InputEvent};
use logger::Logger;
use display::Display;

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
pub static INPUT_POINT: extern "C" fn(*mut Context<'static>, input: InputEvent) -> i32 = input_point;

extern "Rust" {
    fn main() -> i32;
    fn update(system: &mut UserSpace) -> i32; // TODO pass 'Resources setup in main to this update function'
    fn input(system: &mut UserSpace, input: InputEvent) -> i32;
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

    unsafe { update(&mut USERSPACE) }
}

#[no_mangle]
/// Calls the user update function
pub extern "C" fn input_point(raw_ctx: *mut Context<'static>, state: InputEvent) -> i32 {
    // Turn the pointer into a reference and store in a static.
    unsafe {
        CONTEXT_POINTER = Some(&mut *raw_ctx);
    };

    unsafe { input(&mut USERSPACE, state) }
}

static mut USERSPACE: UserSpace = UserSpace {
    display: Display::new(),
    logger: Logger::new(),
};

#[repr(C)]
pub struct UserSpace {
    pub display: Display,
    pub logger: Logger,
}


#[cfg(not(feature = "panic-log"))]
#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

#[cfg(feature = "panic-log")]
#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {

    unsafe { USERSPACE.logger.log_fmt(format_args!("{:?}", info)) };

    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}