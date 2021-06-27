//! Display commands

use display_interface::{DataFormat::U8, DisplayError, WriteOnlyDataCommand};

/// IST7920 Commands

/// Commands
/// TODO: Implement full command support
/// TODO: Implement read write
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum Command {
    /// AY address
    AYAddress(u8),
    /// Bias
    Bias(u8),
    /// Voltage generator clock frequency select
    VoltageClock(u8),
    /// Power control
    /// TODO: Add independent bit fields
    PowerControl(u8),
    /// Turn display on or off.
    DisplayOn(bool),
    /// Set start line
    StartLine(u8),
    /// Display control
    /// SHL: Select (Common Output Mode Select)
    /// 	SHL = 0: COM0 -> COM(N-1)
    /// 	SHL = 1: COM(N-1) -> COM0
    /// ADC: Display RAM Address Mapping
    /// 	ADC = 0: SEG0 -> SEG127
    /// 	ADC = 1: SEG127 -> SEG0
    /// EON: Force the whole LCD point to be turned on regardless of RAM context
    /// 	EON = 0: Normal display
    /// 	EON = 1: Entire display ON
    /// REV: Reverse the lit and until display relation between RAM bit data and LCD cell
    /// 	REV = 0: LCD cell = RAM bit data
    /// 	REV = 1: LCD cell = inverse of RAM bit data
    DisplayControl(bool, bool, bool, bool),
    /// AY Window: start, end
    AYWindow(u8, u8),
    /// AX Window: start, end
    AXWindow(u8, u8),
    /// Software Reset
    SWReset,
    /// Duty
    Duty(u8),
    /// Contrast (Reference volatage select)
    Contrast(u8),
    /// AX Address
    AXAddress(u8),
    /// Booster,
    Booster(Booster),
}

impl Command {
    /// Send command to IST7920
    pub fn send<DI>(self, iface: &mut DI) -> Result<(), DisplayError>
    where
        DI: WriteOnlyDataCommand,
    {
        // Transform command into a fixed size array of 3 u8 and the real length for sending
        let (data, len) = match self {
            Command::AYAddress(address) => ([0x01, address & 0x1f, 0], 2),
            Command::Bias(bias) => ([0x30, bias & 0x3f, 0], 2),
            Command::VoltageClock(clock) => ([0x31, clock & 0x3f, 0], 2),
            Command::PowerControl(control) => ([0x33, control, 0], 2),
            Command::DisplayOn(on) => ([0x3c | (on as u8), 0, 0], 1),
            Command::StartLine(start) => ([0x40, start, 0], 2),
            Command::DisplayControl(shl, adc, eon, rev) => (
                [
                    0x60 | (shl as u8) << 3 | (adc as u8) << 2 | (eon as u8) << 1 | (rev as u8),
                    0,
                    0,
                ],
                1,
            ),
            Command::AYWindow(start, end) => ([0x74, start & 0x1f, end & 0x1f], 3),
            Command::AXWindow(start, end) => ([0x75, start & 0x7f, end & 0x7f], 3),
            Command::SWReset => ([0x76, 0, 0], 1),
            Command::Duty(duty) => ([0x90, duty, 0], 2),
            Command::Contrast(contrast) => ([0xb1, contrast, 0], 2),
            Command::AXAddress(address) => ([0xc0, address & 0x7f, 0], 2),
            Command::Booster(booster) => ([0xfc | (0x03 & (booster as u8)), 0, 0], 1),
        };

        // Send command over the interface
        iface.send_commands(U8(&data[0..len]))
    }
}

/// Booster
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum Booster {
    /// 2*VDD2-0.3
    VddX2 = 0b00,
    /// 3*VDD2-0.3
    VddX3 = 0b01,
    /// 4*VDD2-0.3
    VddX4 = 0b10,
    /// 5*VDD2-0.3
    VddX5 = 0b11,
}
