#![no_std]
//! Generic SPI interface for display drivers
pub mod error;

use error::Error;

use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};
use display_interface_spi::SPIInterfaceNoCS;
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
    pub fn init<DELAY>(&mut self, delay: &mut DELAY)
    where
        DELAY: DelayMs<u8>,
    {
        self.interface.send_commands(U8(&[0x76])); // Software reset.
        delay.delay_ms(50);
        self.interface.send_commands(U8(&[0x3c])); // Display Off
        self.interface.send_commands(U8(&[0x90, 128])); // Set Duty
        self.interface.send_commands(U8(&[0x30, 16])); // Set Bias
        self.interface.send_commands(U8(&[0x31, 0x3f])); // Set voltage generate clock
        self.interface.send_commands(U8(&[0x33, 0x20])); // Power control
        delay.delay_ms(100);
        self.interface.send_commands(U8(&[0x33, 0x2c])); // Power control
        delay.delay_ms(100);
        self.interface.send_commands(U8(&[0xfd])); // Set booster
        delay.delay_ms(100);
        self.interface.send_commands(U8(&[0x33, 0x2f])); // Power control
        delay.delay_ms(200);
        self.interface.send_commands(U8(&[0x064])); // Display Ctrl: Bit3: SHL 2:ADC 1:EON, 0:REV

        self.interface.send_commands(U8(&[0x074, 0x000, 0x00f])); // AY Window
        self.interface.send_commands(U8(&[0x075, 0x000, 0x07f])); // AX Window

        self.interface.send_commands(U8(&[0x040, 64])); // Start line at 64

        self.interface.send_commands(U8(&[0x0b1, 110])); // electronic volume

        self.interface.send_commands(U8(&[0x3d])); // Display on
    }

    /// Send a raw buffer to the display.
    pub fn draw(&mut self, buffer: &[u8]) -> Result<(), DisplayError> {
        self.interface.send_data(U8(&buffer))
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
