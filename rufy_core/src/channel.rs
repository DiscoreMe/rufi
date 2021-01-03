// use tokio::sync::mpsc;
use std::sync::mpsc;
pub type Sender = mpsc::Sender<String>;
pub type Receiver = mpsc::Receiver<String>;

pub fn create_channel() -> (Sender, Receiver) {
    mpsc::channel()
}