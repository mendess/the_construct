use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Message {
    PlayRequest {
        timestamp: Duration,
        sender_time: DateTime<Utc>,
    },
    Pause {
        timestamp: Duration,
        sender_time: DateTime<Utc>,
    },
    PlayCommand {
        timestamp: Duration,
        deadline: DateTime<Utc>
    },
    JoinPause,
}

impl Message {
    pub fn play_request(timestamp: Duration) -> Self {
        Self::PlayRequest {
            timestamp,
            sender_time: Utc::now(),
        }
    }

    pub fn play_command(timestamp: Duration) -> Self {
        Self::PlayCommand {
            timestamp,
            deadline: Utc::now() + chrono::Duration::seconds(10),
        }
    }

    pub fn pause(timestamp: Duration) -> Self {
        Self::Pause {
            timestamp,
            sender_time: Utc::now(),
        }
    }

    pub fn join() -> Self {
        Self::JoinPause
    }
}
