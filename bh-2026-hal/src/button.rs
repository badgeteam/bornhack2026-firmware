use bh_2026_traits::button::{ButtonQueue, ButtonType, JoystickAction};
use embassy_futures::select::select_array;
use embassy_nrf::gpio::Input;

#[embassy_executor::task]
pub async fn button_task(mut buttons: Buttons) {
    buttons.watch_buttons().await;
}

pub struct Buttons {
    button_queue: ButtonQueue,
    execute: Input<'static>,
    cancel: Input<'static>,
    up: Input<'static>,
    down: Input<'static>,
    left: Input<'static>,
    right: Input<'static>,
    fire: Input<'static>,
}

impl Buttons {
    pub fn new(
        button_queue: ButtonQueue,
        execute: Input<'static>,
        cancel: Input<'static>,
        up: Input<'static>,
        down: Input<'static>,
        left: Input<'static>,
        right: Input<'static>,
        fire: Input<'static>,
    ) -> Self {
        Self {
            button_queue,
            execute,
            cancel,
            up,
            down,
            left,
            right,
            fire,
        }
    }

    pub async fn watch_buttons(&mut self) {
        loop {
            match select_array([
                self.execute.wait_for_falling_edge(),
                self.cancel.wait_for_falling_edge(),
                self.up.wait_for_falling_edge(),
                self.down.wait_for_falling_edge(),
                self.left.wait_for_falling_edge(),
                self.right.wait_for_falling_edge(),
                self.fire.wait_for_falling_edge(),
            ])
            .await
            {
                (_, 0) => {
                    self.button_queue.publish_button(ButtonType::Execute).await;
                }
                (_, 1) => {
                    self.button_queue.publish_button(ButtonType::Cancel).await;
                }
                (_, 2) => {
                    self.button_queue
                        .publish_button(ButtonType::Joystick(JoystickAction::Up))
                        .await;
                }
                (_, 3) => {
                    self.button_queue
                        .publish_button(ButtonType::Joystick(JoystickAction::Down))
                        .await;
                }
                (_, 4) => {
                    self.button_queue
                        .publish_button(ButtonType::Joystick(JoystickAction::Left))
                        .await;
                }
                (_, 5) => {
                    self.button_queue
                        .publish_button(ButtonType::Joystick(JoystickAction::Right))
                        .await;
                }
                (_, 6) => {
                    self.button_queue
                        .publish_button(ButtonType::Joystick(JoystickAction::Fire))
                        .await;
                }
                (_, _) => unreachable!(),
            }
        }
    }
}
