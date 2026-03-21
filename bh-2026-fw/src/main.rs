#![no_std]
#![no_main]

use embassy_executor::Spawner;

use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    defmt::info!("Bornhack 2026 firmware");

    bh_2026_hal::init(spawner).await;
}
