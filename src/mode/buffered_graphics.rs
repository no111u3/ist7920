use crate::Ist7920;
use display_interface::{DisplayError, WriteOnlyDataCommand};

#[derive(Debug, Clone)]
pub struct BufferedGraphicsMode {
    buffer: [u8; 128 * 128 / 8],
}

impl BufferedGraphicsMode {
    /// Create a new buffered graphics mode instance
    pub(crate) fn new() -> Self {
        Self {
            buffer: [0; 128 * 128 / 8],
        }
    }
}

impl<DI> Ist7920<DI, BufferedGraphicsMode>
where
    DI: WriteOnlyDataCommand,
{
    /// Clear the display buffer. You need to call `disp.flush()` for any effect on the screen.
    pub fn clear(&mut self) {
        self.mode.buffer = [0; 128 * 128 / 8];
    }

    /// Write out data to the display.
    /// TODO: Rewrite to more efficient implementation
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        Self::flush_buffer_chunks(&mut self.interface, &self.mode.buffer)
    }

    /// Turn a pixel on or off. A non-zero `value` is treated as on, `0` as off. If the X and Y
    /// coordinates are out of the bounds of the display, this method call is a noop.
    pub fn set_pixel(&mut self, x: u32, y: u32, value: bool) {
        let value = value as u8;

        let idx = ((y as usize) / 8 * 128 as usize) + (x as usize);
        let bit = y % 8;

        if let Some(byte) = self.mode.buffer.get_mut(idx) {
            // Set pixel value in byte
            *byte = *byte & !(1 << bit) | (value << bit)
        }
    }
}

#[cfg(feature = "graphics")]
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::Size,
    geometry::{Dimensions, OriginDimensions},
    pixelcolor::BinaryColor,
    Pixel,
};

#[cfg(feature = "graphics")]
impl<DI> DrawTarget for Ist7920<DI, BufferedGraphicsMode>
where
    DI: WriteOnlyDataCommand,
{
    type Color = BinaryColor;
    type Error = DisplayError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let bb = self.bounding_box();

        pixels
            .into_iter()
            .filter(|Pixel(pos, _color)| bb.contains(*pos))
            .for_each(|Pixel(pos, color)| {
                self.set_pixel(pos.x as u32, pos.y as u32, color.is_on())
            });

        Ok(())
    }
}

#[cfg(feature = "graphics")]
impl<DI> OriginDimensions for Ist7920<DI, BufferedGraphicsMode>
where
    DI: WriteOnlyDataCommand,
{
    fn size(&self) -> Size {
        Size::new(128, 128)
    }
}
