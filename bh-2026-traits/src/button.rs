use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    pubsub::{self, PubSubChannel, Publisher, Subscriber},
};

pub type ButtonQueueType = Subscriber<'static, CriticalSectionRawMutex, ButtonType, 10, 5, 1>;

static BUTTON_CHANNEL: PubSubChannel<CriticalSectionRawMutex, ButtonType, 10, 5, 1> =
    PubSubChannel::new();

#[derive(Clone, defmt::Format)]
pub enum JoystickAction {
    Up,
    Down,
    Left,
    Right,
    Fire,
}

#[derive(Clone, defmt::Format)]
pub enum ButtonType {
    Cancel,
    Execute,
    Joystick(JoystickAction),
}

/// The queue for button events
pub struct ButtonQueue {
    publisher: Publisher<'static, CriticalSectionRawMutex, ButtonType, 10, 5, 1>,
}

impl ButtonQueue {
    pub async fn new() -> Self {
        Self {
            publisher: BUTTON_CHANNEL
                .publisher()
                .expect("ButtonQueue already created"),
        }
    }

    pub async fn publish_button(&mut self, button: ButtonType) {
        self.publisher.publish(button).await;
    }
}

#[allow(async_fn_in_trait)]
pub trait ButtonPublisher {
    async fn publish_button(&mut self, button_queue: ButtonQueue);
}

pub struct ButtonSubscriber {}

impl ButtonSubscriber {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_button_queue(&self) -> Result<ButtonQueueType, pubsub::Error> {
        BUTTON_CHANNEL.subscriber()
    }
}
