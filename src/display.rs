use core::result::Result;
use embassy_nrf::{Peri, gpio, interrupt, spim};

// TODO: Make this a struct

fn display_tx<SPI, SCK, DATA>(
    irqs: impl interrupt::typelevel::Binding<SPI::Interrupt, spim::InterruptHandler<SPI>> + 'static,
    epd_spi: &mut Peri<'static, SPI>,
    epd_sck: &mut Peri<'static, SCK>,
    epd_data: &mut Peri<'static, DATA>,
    epd_bus_config: spim::Config,
    data: &[u8],
) -> Result<(), spim::Error>
where
    SPI: spim::Instance,
    SCK: gpio::Pin,
    DATA: gpio::Pin,
{
    let mut epd_spim = spim::Spim::new_txonly(
        epd_spi.reborrow(),
        irqs,
        epd_sck.reborrow(),
        epd_data.reborrow(),
        epd_bus_config,
    );
    epd_spim.blocking_write(data)
}

fn display_rx<SPI, SCK, DATA>(
    irqs: impl interrupt::typelevel::Binding<SPI::Interrupt, spim::InterruptHandler<SPI>> + 'static,
    epd_spi: &mut Peri<'static, SPI>,
    epd_sck: &mut Peri<'static, SCK>,
    epd_data: &mut Peri<'static, DATA>,
    epd_bus_config: spim::Config,
    data: &mut [u8],
) -> Result<(), spim::Error>
where
    SPI: spim::Instance,
    SCK: gpio::Pin,
    DATA: gpio::Pin,
{
    let mut epd_spim = spim::Spim::new_rxonly(
        epd_spi.reborrow(),
        irqs,
        epd_sck.reborrow(),
        epd_data.reborrow(),
        epd_bus_config,
    );
    epd_spim.blocking_read(data)
}

pub fn display_init<SPI, SCK, DATA>(
    irqs: impl interrupt::typelevel::Binding<SPI::Interrupt, spim::InterruptHandler<SPI>> + Clone + 'static,
    epd_spi: &mut Peri<'static, SPI>,
    epd_sck: &mut Peri<'static, SCK>,
    epd_data: &mut Peri<'static, DATA>,
    epd_bus_config: spim::Config,
) -> Result<(), spim::Error>
where
    SPI: spim::Instance,
    SCK: gpio::Pin,
    DATA: gpio::Pin,
{
    let tx_data = [0xde, 0xad, 0xbe, 0xef];

    display_tx(irqs.clone(), epd_spi, epd_sck, epd_data, epd_bus_config.clone(), &tx_data)?;

    let mut rx_data = [0u8; 32];

    display_rx(irqs, epd_spi, epd_sck, epd_data, epd_bus_config, &mut rx_data)?;

    Ok(())
}
