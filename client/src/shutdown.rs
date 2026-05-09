use tokio::sync::watch::{Receiver, Sender};

#[derive(Copy, Clone, Debug)]
pub enum ShutdownReason {
    Key,
    Connection,
    TermSize,
    ServerShutdown,
}

pub struct ShutdownChannel {
    sender: Sender<Option<ShutdownReason>>,
    receiver: Receiver<Option<ShutdownReason>>,
}

impl ShutdownChannel {
    pub fn new() -> Self {
        let (sender, receiver) = tokio::sync::watch::channel(None);
        Self { sender, receiver }
    }

    pub fn clone(other: &Self) -> Self {
        Self {
            sender: other.sender.clone(),
            receiver: other.receiver.clone(),
        }
    }

    pub fn shutdown(&self, reason: ShutdownReason) {
        let _ = self.sender.send(Some(reason));
    }

    pub fn is_shutdown(&self) -> bool {
        self.receiver.borrow().is_some()
    }

    pub fn get_reason(&self) -> Option<ShutdownReason> {
        *self.receiver.borrow()
    }
}
