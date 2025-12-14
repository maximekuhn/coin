use chrono::{DateTime, Utc};
use domain::types::user_id::UserId;
use uuid::Uuid;

pub struct Entry {
    pub id: Uuid,
    pub user_id: UserId,
    pub hashed_password: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub sessions: Vec<Session>,
}

pub struct Session {
    pub id: [u8; 128],
    pub entry_id: Uuid,
    pub expires_at: DateTime<Utc>,
}

impl Entry {
    /// Returns the session that expires the soonest.
    pub fn oldest_session(&self) -> Option<&Session> {
        self.sessions.iter().min_by_key(|s| s.expires_at)
    }
}

impl Session {
    fn is_valid(&self, now: DateTime<Utc>) -> bool {
        now < self.expires_at
    }

    pub fn is_expired(&self, now: DateTime<Utc>) -> bool {
        !self.is_valid(now)
    }
}
