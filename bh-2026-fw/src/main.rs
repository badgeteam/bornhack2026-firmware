#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;

use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Bornhack 2026 firmware");

    let peripherals = bh_2026_hal::init(spawner).await;
    let mut queue = peripherals.button.get_button_queue().expect("Could not get queue");

    loop {
        let button = queue.next_message_pure().await;
        info!("Button: {}", button);
    }
}
