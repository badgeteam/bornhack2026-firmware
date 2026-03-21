pub enum LedColour {
    Red,
    Green,
    Blue,
}

#[allow(async_fn_in_trait)]
pub trait Led {
    async fn set_brightness(&mut self, led: LedColour, brightness: u16);
}
