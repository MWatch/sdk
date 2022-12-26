use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::{raw::RawU16, Rgb565},
    prelude::{Dimensions, OriginDimensions, Size, RawData},
    Pixel,
};

use crate::MWatchABI;

#[repr(C)]
pub struct Display {
    _0: (), // TODO store useful info from SetupFN here!! like size, rotation etc
}

impl Display {
    pub const fn new() -> Self {
        Display { _0: () }
    }
}

impl DrawTarget for Display {
    type Color = Rgb565;

    type Error = ();

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = embedded_graphics::Pixel<Self::Color>>,
    {
        let table = MWatchABI::table();
        let ctx = MWatchABI::context();

        let bb = self.bounding_box();

        pixels
            .into_iter()
            .filter(|Pixel(pos, _)| bb.contains(*pos))
            .for_each(|Pixel(pos, color)| {
                let x = pos.x;
                let y = pos.y;
                let color: u16 = RawU16::from(color).into_inner();
                unsafe {
                    (table.draw_pixel)(ctx, x as u8, y as u8, color);
                }
            });

        Ok(())
    }
}

impl OriginDimensions for Display {
    fn size(&self) -> embedded_graphics::prelude::Size {
        Size::new(128, 128) // TODO remove hard coded
    }
}
