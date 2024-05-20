use std::time::Duration;

use chrono::{DateTime, Local};
use tokio::{task::JoinHandle, time};
use uuid::Uuid;

// A client connection works like a session
pub struct ClientConnection {
    pub id: Uuid,
    pub closed: bool,
    _creation_time: DateTime<Local>,
    lifetime: Duration,
    expiration_timeout: Option<JoinHandle<()>>,
}

impl ClientConnection {
    pub async fn create(creation_time: DateTime<Local>, lifetime: Duration) -> Self {
        let mut new_session = Self {
            id: Uuid::new_v4(),
            _creation_time: creation_time,
            lifetime,
            expiration_timeout: None,
            closed: false,
        };

        new_session.reset_expiration_timeout();
        new_session
    }

    fn reset_expiration_timeout(&mut self) {
        if let Some(expiration_timeout) = &self.expiration_timeout {
            expiration_timeout.abort();
        };
        unsafe {
            let this = &mut *(self as *mut Self);
            let timeout_duration = self.lifetime;
            self.expiration_timeout = Some(tokio::spawn(async move {
                time::sleep(timeout_duration).await;
                this.closed = true;
            }));
        };
    }
}
