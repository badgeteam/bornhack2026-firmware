#![no_std]

use crate::{button::ButtonSubscriber, led::Led};

pub mod led;
pub mod button;

pub struct BadgePeripherals<LED: Led> {
    pub led: LED,
    pub button: ButtonSubscriber,
}
