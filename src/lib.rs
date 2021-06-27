#![no_std]
//! Generic SPI interface for display drivers
mod command;
mod error;

use command::{Booster, Command};
use error::Error;

use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};
use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};

/// IST7920 LCD display driver.
#[derive(Copy, Clone, Debug)]
pub struct Ist7920<DI> {
    interface: DI,
}

impl<DI> Ist7920<DI>
where
    DI: WriteOnlyDataCommand,
{
    /// Create a IST7920 interface
    pub fn new(interface: DI) -> Self {
        Self { interface }
    }

    /// Initialise the display in one of the available addressing modes.
    /// TODO: Add address setup
    pub fn init<DELAY>(&mut self, delay: &mut DELAY) -> Result<(), DisplayError>
    where
        DELAY: DelayMs<u8>,
    {
        Command::SWReset.send(&mut self.interface)?;
        delay.delay_ms(50);
        Command::DisplayOn(false).send(&mut self.interface)?;
        Command::Duty(128).send(&mut self.interface)?;
        Command::Bias(16).send(&mut self.interface)?;
        Command::VoltageClock(0x3f).send(&mut self.interface)?;
        Command::PowerControl(0x20).send(&mut self.interface)?;
        delay.delay_ms(100);
        Command::PowerControl(0x2c).send(&mut self.interface)?;
        delay.delay_ms(100);
        Command::Booster(Booster::VddX3).send(&mut self.interface)?;
        delay.delay_ms(100);
        Command::PowerControl(0x2f).send(&mut self.interface)?;
        delay.delay_ms(200);
        Command::DisplayControl(false, true, false, false).send(&mut self.interface)?;

        Command::AYWindow(0x0, 0xf).send(&mut self.interface)?;
        Command::AXWindow(0x0, 0x7f).send(&mut self.interface)?;

        Command::StartLine(64).send(&mut self.interface)?;
        Command::AYAddress(0).send(&mut self.interface)?;
        Command::AXAddress(0).send(&mut self.interface)?;

        Command::Contrast(110).send(&mut self.interface)?;

        Command::DisplayOn(true).send(&mut self.interface)?;

        Ok(())
    }

    /// Send a raw buffer to the display.
    pub fn draw(&mut self, buffer: &[u8]) -> Result<(), DisplayError> {
        self.interface.send_data(U8(&buffer))
    }

    /// Turn the display on or off. The display can be drawn to and retains all
    /// of its memory even while off.
    pub fn set_display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        Command::DisplayOn(on).send(&mut self.interface)
    }

    /// Set the position in the framebuffer of the display limiting where any sent data should be
    /// drawn. This method can be used for changing the affected area on the screen as well
    /// as (re-)setting the start point of the next `draw` call.
    pub fn set_draw_area(&mut self, start: (u8, u8), end: (u8, u8)) -> Result<(), DisplayError> {
        Command::AYWindow(start.0 / 8, end.0 / 8).send(&mut self.interface)?;

        Command::AXWindow(start.0, end.0).send(&mut self.interface)?;

        Command::AXAddress(start.1).send(&mut self.interface)?;
        Command::AYAddress(start.0 / 8).send(&mut self.interface)?;

        Ok(())
    }

    /// Reset the display.
    pub fn reset<RST, DELAY, PinE>(
        &mut self,
        rst: &mut RST,
        delay: &mut DELAY,
    ) -> Result<(), Error<(), PinE>>
    where
        RST: OutputPin<Error = PinE>,
        DELAY: DelayMs<u8>,
    {
        inner_reset(rst, delay)
    }
}

fn inner_reset<RST, DELAY, PinE>(rst: &mut RST, delay: &mut DELAY) -> Result<(), Error<(), PinE>>
where
    RST: OutputPin<Error = PinE>,
    DELAY: DelayMs<u8>,
{
    rst.set_high().map_err(Error::Pin)?;
    delay.delay_ms(1);
    rst.set_low().map_err(Error::Pin)?;
    delay.delay_ms(10);
    rst.set_high().map_err(Error::Pin)?;
    delay.delay_ms(20);

    Ok(())
}
