//! Display modes

pub mod buffered_graphics;

pub use buffered_graphics::BufferedGraphicsMode;

use crate::Ist7920;

use display_interface::{DisplayError, WriteOnlyDataCommand};

pub struct BasicMode;

impl<DI> Ist7920<DI, BasicMode>
where
    DI: WriteOnlyDataCommand,
{
    /// Clear the display
    pub fn clear(&mut self) -> Result<(), DisplayError> {
        self.set_draw_area((0, 0), (127, 127))?;

        for _ in 0..(128 * 128 / 32) {
            self.draw(&[0x00, 0x00, 0x00, 0x00])?;
        }

        Ok(())
    }
}
