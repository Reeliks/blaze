use std::time::Duration;

use chrono::{DateTime, Local};
use uuid::Uuid;

// A client connection works like a session
pub struct ClientSession {
    pub id: Uuid,
    pub _creation_time: DateTime<Local>,
    pub expiration_time: Option<DateTime<Local>>,
}

impl ClientSession {
    pub fn new(creation_time: DateTime<Local>) -> Self {
        Self {
            id: Uuid::new_v4(),
            _creation_time: creation_time,
            expiration_time: None,
        }
    }

    pub fn is_active(&self) -> bool {
        if self.expiration_time.is_none() {
            return true;
        }
        Local::now() < self.expiration_time.unwrap()
    }

    pub fn activate_indefinitely(&mut self) {
        self.expiration_time = None
    }

    pub fn desactivate(&mut self) {
        self.expiration_time = Some(Local::now())
    }

    pub fn activate_temporairly(&mut self, lifetime: Duration) {
        self.expiration_time = Some(Local::now() + lifetime)
    }
}
