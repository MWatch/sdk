

use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::PixelColorU16;
use embedded_graphics::drawable;
use mwatch_kernel_api::{Context, CALLBACK_TABLE};


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

    pub const fn new() -> Self {
        Display { _0: 0u8 }
    }
}
