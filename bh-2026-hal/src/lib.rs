#![no_std]

mod board;
mod display;
mod display_interface;

use embassy_executor::Spawner;
use embassy_nrf::{Peri, gpio::{Input, Level, Output, OutputDrive, Pull}};
use embassy_nrf::{bind_interrupts, peripherals, spim};
use embassy_time::Delay;

use crate::{display::Display, display_interface::BidirectionalSpi};

bind_interrupts!(struct Irqs {
    SPIM3 => spim::InterruptHandler<peripherals::SPI3>;
});

// When you are okay with using a nightly compiler it's better to use https://docs.rs/static_cell/2.1.0/static_cell/macro.make_static.html
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

pub async fn init(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    // LEDs
    let led_red = Output::new(board!(p, led_red), Level::High, OutputDrive::Standard);
    let led_green = Output::new(board!(p, led_green), Level::High, OutputDrive::Standard);
    let led_blue = Output::new(board!(p, led_blue), Level::High, OutputDrive::Standard);

    // Buttons
    let btn_can = Input::new(board!(p, btn_can), Pull::Up);
    let btn_exe = Input::new(board!(p, btn_exe), Pull::Up);

    // EPD dislay
    let mut epd_bus_config = spim::Config::default();
    epd_bus_config.frequency = spim::Frequency::M16;

    let epd_spi = mk_static!(Peri<'static, peripherals::SPI3>, board!(p, epd_spi));
    let epd_sck = mk_static!(Peri<'static, peripherals::P0_08>, board!(p, epd_sck));
    let epd_data = mk_static!(Peri<'static, peripherals::P0_27>, board!(p, epd_mosi));

    let epd_csn = Output::new(board!(p, epd_csn), Level::High, OutputDrive::Standard);
    let epd_dc = Output::new(board!(p, epd_dc), Level::High, OutputDrive::Standard);
    let epd_reset = Output::new(board!(p, epd_reset), Level::Low, OutputDrive::Standard);
    let epd_busy = Input::new(board!(p, epd_busy), Pull::Up);

    let interface = BidirectionalSpi::new(Irqs, epd_spi, epd_sck, epd_data, epd_bus_config);
    let mut display = Display::new(Delay, interface, epd_csn, epd_dc, epd_reset, epd_busy);

    display.reset().await.expect("Could not reset the display");

    display.init().await.expect("Could not init display");
}
