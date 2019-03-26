
use mwatch_kernel_lib::types::{Context, CALLBACK_TABLE};

#[repr(C)]
pub struct Logger {
    _0: u8
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

    pub const fn new() -> Self {
        Logger { _0: 0u8 }
    }
}

impl core::fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let ctx = Context::get();
        unsafe {
            (CALLBACK_TABLE.print)(ctx, s);
        }
        Ok(())
    }
}