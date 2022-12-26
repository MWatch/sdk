#![no_std]

mod display;
mod logger;
pub mod prelude;

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};
use display::Display;
use logger::Logger;
use mwatch_kernel::application::{Context, InputFn, ServiceFn, SetupFn, Table};
use mwatch_kernel::system::input::InputEvent;

#[link_section = ".entry_point"]
#[no_mangle]
/// First 4 bytes of the binary as a function ptr
/// The pointer the watch calls to start running this application.
pub static ENTRY_POINT: SetupFn = entry_point;

#[link_section = ".update_point"]
#[no_mangle]
/// Second 4 byte function ptr
/// The pointer the watch calls to start running this application.
pub static UPDATE_POINT: ServiceFn = update_point;

#[link_section = ".input_point"]
#[no_mangle]
/// Final 4 byte function ptr
/// The pointer the watch calls to handle input
pub static INPUT_POINT: InputFn = input_point;

extern "Rust" {
    fn main() -> i32;
    fn update(system: &mut UserSpace, display: &mut Display) -> i32;
    fn input(system: &mut UserSpace, input: InputEvent) -> i32;
}

#[no_mangle]
/// The function called by the host to start us up. Does some setup, then
/// jumps to a function called `main` defined by the actual application using
/// this crate.
pub unsafe extern "C" fn entry_point(table: *mut Table) -> i32 {
    MWABI.set_table(table);
    // TODO pass hw info here
    main()
}

#[no_mangle]
/// Calls the user update function
pub unsafe extern "C" fn update_point(raw_ctx: *mut Context) -> i32 {
    // Turn the pointer into a reference and store in a static.
    MWABI.set_context(raw_ctx);

    update(
        &mut UserSpace {
            logger: Logger::new(),
        },
        &mut Display::new(),
    )
}

#[no_mangle]
/// Calls the user update function
pub unsafe extern "C" fn input_point(raw_ctx: *mut Context, state: InputEvent) -> i32 {
    // Turn the pointer into a reference and store in a static.
    MWABI.set_context(raw_ctx);

    input(
        &mut UserSpace {
            logger: Logger::new(),
        },
        state,
    )
}

static mut MWABI: MWatchABI = MWatchABI {
    context: core::ptr::null_mut(),
    table: core::ptr::null_mut(),
};

pub(crate) struct MWatchABI {
    context: *mut Context,
    table: *mut Table,
}

impl MWatchABI {
    
    pub(crate) fn context() -> &'static mut Context {
        unsafe {
            debug_assert!(!MWABI.context.is_null());
            &mut *MWABI.context
        }
    }

    pub(crate) fn table() -> &'static mut Table {
        unsafe {
            debug_assert!(!MWABI.table.is_null());
            &mut *MWABI.table
        }
    }

    pub(crate) fn set_context(&mut self, context: *mut Context) {
        self.context = context;
    }

    pub(crate) fn set_table(&mut self, table: *mut Table) {
        self.table = table;
    }
}

#[repr(C)]
/// User space api abstraction
pub struct UserSpace {
    pub logger: Logger,
}

#[cfg(not(any(feature = "panic-log", feature = "panic-simple")))]
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
    unsafe {
        UserSpace {
            logger: Logger::new(),
        }
        .logger
        .log_fmt(format_args!("{:?}", info))
    };

    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}

#[cfg(feature = "panic-simple")]
#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    UserSpace {
        logger: Logger::new(),
    }
    .logger
    .log_str("application panicked");

    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
