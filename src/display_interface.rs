use core::result::Result;
use embassy_nrf::{
    Peri,
    gpio::{self},
    interrupt, spim,
};
use embedded_hal::spi::{Error, ErrorKind, ErrorType};

// TODO: Reuse the interface when doing consecutive tx and rx

#[derive(Debug)]
pub enum BidirectionalSpiError {
    SpiError(spim::Error),
    TransferNotAllowed,
}

impl Error for BidirectionalSpiError {
    fn kind(&self) -> ErrorKind {
        match self {
            BidirectionalSpiError::SpiError(err) => match err {
                spim::Error::BufferNotInRAM => ErrorKind::Other,
                _ => ErrorKind::Other,
            },
            BidirectionalSpiError::TransferNotAllowed => ErrorKind::Other,
        }
    }
}

impl From<spim::Error> for BidirectionalSpiError {
    fn from(value: spim::Error) -> Self {
        Self::SpiError(value)
    }
}

pub struct BidirectionalSpi<SPI, SCK, DATA, IRQS>
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

impl<SPI, SCK, DATA, IRQ> BidirectionalSpi<SPI, SCK, DATA, IRQ>
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

    async fn tx(&mut self, data: &[u8]) -> Result<(), spim::Error>
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
        epd_spim.write(data).await
    }

    async fn rx(&mut self, data: &mut [u8]) -> Result<(), spim::Error> {
        let mut epd_spim = spim::Spim::new_rxonly(
            self.spi.reborrow(),
            self.irqs.clone(),
            self.sck.reborrow(),
            self.data.reborrow(),
            self.bus_config.clone(),
        );
        epd_spim.read(data).await
    }
}

impl<SPI, SCK, DATA, IRQ> ErrorType for BidirectionalSpi<SPI, SCK, DATA, IRQ>
where
    IRQ: interrupt::typelevel::Binding<SPI::Interrupt, spim::InterruptHandler<SPI>> + Clone,
    SPI: spim::Instance,
    SCK: gpio::Pin,
    DATA: gpio::Pin,
{
    type Error = BidirectionalSpiError;
}

impl<SPI, SCK, DATA, IRQ> embedded_hal_async::spi::SpiBus for BidirectionalSpi<SPI, SCK, DATA, IRQ>
where
    IRQ: interrupt::typelevel::Binding<SPI::Interrupt, spim::InterruptHandler<SPI>> + Clone,
    SPI: spim::Instance,
    SCK: gpio::Pin,
    DATA: gpio::Pin,
{
    async fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.rx(words).await?;
        Ok(())
    }

    async fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.tx(words).await?;
        Ok(())
    }

    async fn transfer(&mut self, _read: &mut [u8], _write: &[u8]) -> Result<(), Self::Error> {
        Err(BidirectionalSpiError::TransferNotAllowed)
    }

    async fn transfer_in_place(&mut self, _words: &mut [u8]) -> Result<(), Self::Error> {
        Err(BidirectionalSpiError::TransferNotAllowed)
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
