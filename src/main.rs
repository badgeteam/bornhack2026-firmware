#![no_std]
#![no_main]

mod board;
mod display;

use crate::display::display_init;
use embassy_executor::Spawner;
use embassy_nrf::{
    Peri,
    gpio::{AnyPin, Input, Level, Output, OutputDrive, Pull},
};
use embassy_time::Timer;

use {defmt_rtt as _, panic_probe as _};

// SPI
use embassy_nrf::{bind_interrupts, peripherals, spim};
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

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    defmt::info!("Bornhack 2026 firmware");
    let p = embassy_nrf::init(Default::default());

    // LEDs
    let led_red = Output::new(board!(p, led_red), Level::High, OutputDrive::Standard);
    let led_green = Output::new(board!(p, led_green), Level::High, OutputDrive::Standard);
    let led_blue = Output::new(board!(p, led_blue), Level::High, OutputDrive::Standard);

    // Buttons
    let btn_can = Input::new(board!(p, btn_can), Pull::Up);
    let btn_exe = Input::new(board!(p, btn_exe), Pull::Up);

    spawner.spawn(blink_led_task(led_blue)).unwrap();
    spawner.spawn(button_task(btn_exe, led_green)).unwrap();
    spawner.spawn(button_task(btn_can, led_red)).unwrap();

    // EPD dislay
    let mut epd_bus_config = spim::Config::default();
    epd_bus_config.frequency = spim::Frequency::M16;
    let mut epd_chip_select = Output::new(board!(p, epd_csn), Level::High, OutputDrive::Standard);

    let mut epd_spi = mk_static!(Peri<'static, peripherals::SPI3>, board!(p, epd_spi));
    let mut epd_sck = mk_static!(Peri<'static, peripherals::P0_08>, board!(p, epd_sck));
    let mut epd_data = mk_static!(Peri<'static, peripherals::P0_27>, board!(p, epd_data));

    display_init(
        Irqs,
        &mut epd_spi,
        &mut epd_sck,
        &mut epd_data,
        epd_bus_config,
    )
    .expect("Could not init display");

    // spawner.spawn(epd_task(epd_spim, epd_chip_select)).unwrap();

    /*loop {
        led_red.set_low();
        led_green.set_high();
        Timer::after_millis(100).await;
        led_red.set_high();
        led_green.set_low();
        Timer::after_millis(100).await;
    }*/
}

#[embassy_executor::task(pool_size = 3)]
async fn blink_led_task(mut led: Output<'static>) {
    loop {
        led.set_low();
        Timer::after_millis(500).await;
        led.set_high();
        Timer::after_millis(500).await;
    }
}

#[embassy_executor::task(pool_size = 3)]
async fn button_task(mut button: Input<'static>, mut led: Output<'static>) {
    loop {
        button.wait_for_falling_edge().await;
        Timer::after_millis(10).await; // Debounce
        if button.is_high() {
            continue;
        }
        led.set_low();
        Timer::after_millis(500).await;
        led.set_high();
        Timer::after_millis(500).await;
    }
}
