use tokio::sync::watch;

/// Ensures channel is never closed by storing both ends together.
#[derive(Debug, Clone)]
pub struct Channel<T> {
    sender: watch::Sender<T>,
    receiver: watch::Receiver<T>,
}

impl<T> Channel<T> {
    pub fn new(value: T) -> Self {
        let (sender, receiver) = watch::channel(value);
        Channel { sender, receiver }
    }

    pub fn has_changed(&self) -> bool {
        self.receiver
            .has_changed()
            .expect("Invariant: channel can't close")
    }

    pub fn borrow_and_update(&mut self) -> watch::Ref<'_, T> {
        self.receiver.borrow_and_update()
    }

    pub fn sender(&self) -> watch::Sender<T> {
        self.sender.clone()
    }

    pub fn receiver(&self) -> watch::Receiver<T> {
        self.receiver.clone()
    }
}
