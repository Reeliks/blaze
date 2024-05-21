use crossbeam::channel::{bounded, Sender};
use std::thread;

use std::io::Result;

pub struct InfoChannel {
    sender: Option<Sender<String>>,
}

impl InfoChannel {
    pub fn new(sender: Option<Sender<String>>) -> Self {
        InfoChannel { sender }
    }

    pub fn send(self, message: String) -> Result<()> {
        if let Some(channel) = self.sender {
            channel.send(message).unwrap();
        }
        Ok(())
    }
}

impl Clone for InfoChannel {
    fn clone(&self) -> Self {
        InfoChannel {
            sender: self.sender.clone(),
        }
    }
}

pub fn get_console_info_channel() -> InfoChannel {
    let (tx, rx) = bounded(100);
    thread::spawn(move || loop {
        if let Ok(received) = rx.try_recv() {
            println!("{}", received);
        }
    });
    InfoChannel::new(Some(tx))
}
