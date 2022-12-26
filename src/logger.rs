
use crate::MWatchABI;

#[repr(C)]
pub struct Logger {
    _0: ()
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
        Logger { _0: () }
    }
}

impl core::fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let ctx = MWatchABI::context();
        let table = MWatchABI::table();
        unsafe {
            (table.print)(ctx, s.as_ptr(), s.len());
        }
        Ok(())
    }
}