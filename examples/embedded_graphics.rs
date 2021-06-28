#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

use cortex_m::peripheral::Peripherals;

use stm32f4xx_hal::{
    delay::Delay,
    prelude::*,
    spi::{self, Spi},
    stm32,
};

use embedded_graphics::{
    image::Image,
    pixelcolor::{BinaryColor, Rgb565},
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder, Rectangle, Triangle},
};

use tinybmp::Bmp;

use display_interface_spi::SPIInterface;
use ist7920::Ist7920;

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();

    let cp = Peripherals::take().unwrap();

    let gpioa = p.GPIOA.split();
    let gpiob = p.GPIOB.split();
    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze();

    let mut led = gpioa.pa5.into_push_pull_output();
    led.set_low().ok();

    let sck = gpiob.pb3.into_alternate_af5();
    let miso = spi::NoMiso;
    let mosi = gpiob.pb5.into_alternate_af5();

    let dc = gpiob.pb4.into_push_pull_output();
    let mut res = gpiob.pb10.into_push_pull_output();
    let cs = gpiob.pb13.into_push_pull_output();

    let mut delay = Delay::new(cp.SYST, clocks);

    let mode = spi::Mode {
        polarity: spi::Polarity::IdleLow,
        phase: spi::Phase::CaptureOnFirstTransition,
    };

    let spi = Spi::spi1(p.SPI1, (sck, miso, mosi), mode, 8_000_000.hz(), clocks);

    let iface = SPIInterface::new(spi, dc, cs);

    let mut display = Ist7920::new(iface).into_buffered_graphics_mode();

    display.reset(&mut res, &mut delay).ok();

    display.init(&mut delay).ok();

    let bmp: Bmp<Rgb565, 'static> = Bmp::from_slice(include_bytes!("rust.bmp")).unwrap();

    // rust image
    Image::new(&bmp, Point::new(32, 32))
        .draw(&mut display.color_converted())
        .unwrap();

    display.flush().unwrap();

    let yoffset = 50;

    let style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

    let mut select_figure = 0;
    loop {
        delay.delay_ms(500_u16);
        display.clear();
        select_figure = match select_figure {
            0 => {
                // triangle
                Triangle::new(
                    Point::new(16, 16 + yoffset),
                    Point::new(16 + 16, 16 + yoffset),
                    Point::new(16 + 8, yoffset),
                )
                .into_styled(style)
                .draw(&mut display)
                .unwrap();

                // square
                Rectangle::new(Point::new(52, yoffset), Size::new_equal(16))
                    .into_styled(style)
                    .draw(&mut display)
                    .unwrap();

                // circle
                Circle::new(Point::new(88, yoffset), 16)
                    .into_styled(style)
                    .draw(&mut display)
                    .unwrap();
                1
            }
            _ => {
                // rust image
                Image::new(&bmp, Point::new(32, 32))
                    .draw(&mut display.color_converted())
                    .unwrap();
                0
            }
        };

        display.flush().unwrap();

        led.toggle().ok();
    }
}
