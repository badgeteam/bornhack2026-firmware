#![no_std]

mod board;
pub mod button;
mod display;
mod display_interface;
pub mod led;

use bh_2026_traits::{
    BadgePeripherals,
    button::{ButtonQueue, ButtonSubscriber},
};
use embassy_executor::Spawner;
use embassy_nrf::{
    Peri,
    gpio::{Input, Level, Output, OutputDrive, Pull},
    pwm::{self, SimplePwm},
};
use embassy_nrf::{bind_interrupts, peripherals, spim};
use embassy_time::Delay;

use crate::{button::{Buttons, button_task}, display::Display, display_interface::BidirectionalSpi, led::Led};

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

pub async fn init(spawner: Spawner) -> BadgePeripherals<Led> {
    let p = embassy_nrf::init(Default::default());

    let mut led_pwm_config = pwm::SimpleConfig::default();
    led_pwm_config.ch0_idle_level = Level::High;
    led_pwm_config.ch1_idle_level = Level::High;
    led_pwm_config.ch2_idle_level = Level::High;
    let led_pwm = SimplePwm::new_3ch(
        p.PWM0,
        board!(p, led_red),
        board!(p, led_green),
        board!(p, led_blue),
        &led_pwm_config,
    );
    let led = led::Led::new(led_pwm).await;

    // Buttons

    let button_queue = ButtonQueue::new().await;
    let buttons = Buttons::new(
        button_queue,
        Input::new(board!(p, btn_exe), Pull::Up),
        Input::new(board!(p, btn_can), Pull::Up),
        Input::new(board!(p, joy_up), Pull::Up),
        Input::new(board!(p, joy_down), Pull::Up),
        Input::new(board!(p, joy_left), Pull::Up),
        Input::new(board!(p, joy_right), Pull::Up),
        Input::new(board!(p, joy_fire), Pull::Up),
    );
    spawner.spawn(button_task(buttons).unwrap());
    let button = ButtonSubscriber::new();

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

    BadgePeripherals { led, button }
}
