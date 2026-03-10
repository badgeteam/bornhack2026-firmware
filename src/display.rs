use embassy_nrf::{
    gpio::{self, Input, Output},
    interrupt, spim,
};

use crate::display_interface::DisplayInterface;

pub fn display_init<SPI, SCK, DATA, IRQ>(
    interface: &mut DisplayInterface<SPI, SCK, DATA, IRQ>,
    mut epd_csn: Output<'static>,
    mut epd_dc: Output<'static>,
    mut epd_reset: Output<'static>,
    mut epd_busy: Input<'static>,
) -> Result<(), spim::Error>
where
    SPI: spim::Instance,
    SCK: gpio::Pin,
    DATA: gpio::Pin,
    IRQ: interrupt::typelevel::Binding<SPI::Interrupt, spim::InterruptHandler<SPI>> + Clone,
{
    epd_dc.set_low(); // Command mode
    epd_csn.set_low(); // Activate device

    let tx_data = [0x1B];

    interface.tx(&tx_data)?;

    epd_dc.set_high(); // Data mode

    let mut rx_data = [0u8; 2];

    interface.rx(&mut rx_data)?;

    epd_csn.set_high(); // Deactivate device

    defmt::info!("Read from EPD: {=[u8]:#04x}", rx_data);

    Ok(())
}
