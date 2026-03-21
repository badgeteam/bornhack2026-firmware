use core::cell::RefCell;

use bh_2026_traits::led::LedColour;
use embassy_nrf::pwm::{DutyCycle, SimplePwm};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};

static LED_PWM: Mutex<CriticalSectionRawMutex, RefCell<Option<SimplePwm<'static>>>> =
    Mutex::new(RefCell::new(None));

#[derive(Clone)]
pub struct Led {}

impl Led {
    pub(crate) async fn new(led_pwm: SimplePwm<'static>) -> Self {
        if LED_PWM.lock().await.borrow().is_none() {
            LED_PWM.lock().await.replace(Some(led_pwm));
        } else {
            panic!("LED already initialized");
        }

        Self {}
    }
}

impl bh_2026_traits::led::Led for Led {
    async fn set_brightness(&mut self, led: LedColour, brightness: u16) {
        let colour = match led {
            LedColour::Red => 0,
            LedColour::Green => 1,
            LedColour::Blue => 2,
        };

        let pwm = LED_PWM.lock().await;
        pwm.borrow_mut()
            .as_mut()
            .expect("LED driver not initialized")
            .set_duty(colour, DutyCycle::normal(brightness));
    }
}
