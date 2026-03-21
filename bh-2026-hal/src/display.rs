use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal_async::{delay::DelayNs, spi::SpiBus};

#[derive(Debug)]
pub enum DisplayError {
    SpiError,
    PinError,
}

pub struct Display<DELAY, SPI, CSN, DC, RESET, BUSY>
where
    DELAY: DelayNs,
    SPI: SpiBus,
    CSN: OutputPin,
    DC: OutputPin,
    RESET: OutputPin,
    BUSY: InputPin,
{
    delay: DELAY,
    spi: SPI,
    csn: CSN,
    dc: DC,
    reset: RESET,
    busy: BUSY,
}

impl<DELAY, SPI, CSN, DC, RESET, BUSY> Display<DELAY, SPI, CSN, DC, RESET, BUSY>
where
    DELAY: DelayNs,
    SPI: SpiBus,
    CSN: OutputPin,
    DC: OutputPin,
    RESET: OutputPin,
    BUSY: InputPin,
{
    pub fn new(delay: DELAY, spi: SPI, csn: CSN, dc: DC, reset: RESET, busy: BUSY) -> Self {
        Self {
            delay,
            spi,
            csn,
            dc,
            reset,
            busy,
        }
    }

    pub async fn reset(&mut self) -> Result<(), DisplayError> {
        self.reset.set_low().map_err(|_| DisplayError::PinError)?; // Reset
        self.delay.delay_ms(100).await;
        self.reset.set_high().map_err(|_| DisplayError::PinError)?; // Enable
        self.delay.delay_ms(100).await;
        Ok(())
    }

    pub async fn init(&mut self) -> Result<(), DisplayError> {
        self.dc.set_low().map_err(|_| DisplayError::PinError)?; // Command mode
        self.csn.set_low().map_err(|_| DisplayError::PinError)?; // Activate device

        let tx_data = [0x1B];

        self.spi
            .write(&tx_data).await
            .map_err(|_| DisplayError::SpiError)?;

        self.dc.set_high().map_err(|_| DisplayError::PinError)?; // Data mode

        let mut rx_data = [0u8; 2];

        self.spi
            .read(&mut rx_data).await
            .map_err(|_| DisplayError::SpiError)?;

        self.csn.set_high().map_err(|_| DisplayError::PinError)?; // Deactivate device

        defmt::info!("Read from EPD: {=[u8]:#04x}", rx_data);

        Ok(())
    }
}
