use core::result::Result;
use embassy_nrf::{
    Peri,
    gpio::{self},
    interrupt, spim,
};

// TODO: Make this implement embedded-hal traits and reuse the interface when doing consecutive tx and rx

pub struct DisplayInterface<SPI, SCK, DATA, IRQS>
where
    IRQS: interrupt::typelevel::Binding<SPI::Interrupt, spim::InterruptHandler<SPI>> + Clone,
    SPI: spim::Instance,
    SCK: gpio::Pin,
    DATA: gpio::Pin,
{
    irqs: IRQS,
    spi: &'static mut Peri<'static, SPI>,
    sck: &'static mut Peri<'static, SCK>,
    data: &'static mut Peri<'static, DATA>,
    bus_config: spim::Config,
}

impl<SPI, SCK, DATA, IRQ> DisplayInterface<SPI, SCK, DATA, IRQ>
where
    IRQ: interrupt::typelevel::Binding<SPI::Interrupt, spim::InterruptHandler<SPI>> + Clone,
    SPI: spim::Instance,
    SCK: gpio::Pin,
    DATA: gpio::Pin,
{
    pub fn new(
        irqs: IRQ,
        spi: &'static mut Peri<'static, SPI>,
        sck: &'static mut Peri<'static, SCK>,
        data: &'static mut Peri<'static, DATA>,
        bus_config: spim::Config,
    ) -> Self {
        Self {
            spi,
            irqs,
            sck,
            data,
            bus_config,
        }
    }

    pub fn tx(&mut self, data: &[u8]) -> Result<(), spim::Error>
    where
        SPI: spim::Instance,
        SCK: gpio::Pin,
        DATA: gpio::Pin,
    {
        let mut epd_spim = spim::Spim::new_txonly(
            self.spi.reborrow(),
            self.irqs.clone(),
            self.sck.reborrow(),
            self.data.reborrow(),
            self.bus_config.clone(),
        );
        epd_spim.blocking_write(data)
    }

    pub fn rx(&mut self, data: &mut [u8]) -> Result<(), spim::Error> {
        let mut epd_spim = spim::Spim::new_rxonly(
            self.spi.reborrow(),
            self.irqs.clone(),
            self.sck.reborrow(),
            self.data.reborrow(),
            self.bus_config.clone(),
        );
        epd_spim.blocking_read(data)
    }
}
