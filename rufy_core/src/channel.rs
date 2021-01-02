use tokio::sync::mpsc;

pub type Sender = mpsc::Sender<String>;
pub type Receiver = mpsc::Receiver<String>;

pub fn create_channel() -> (Sender, Receiver) {
    mpsc::channel(8)
}

pub fn default_receiver() -> Receiver {
    let (sx, rx) = mpsc::channel(1);
    rx
}