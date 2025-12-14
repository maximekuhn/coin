use chrono::{Duration, Utc};
use domain::types::user_id::UserId;
use rand_core::{OsRng, RngCore};

use crate::auth::{argon2, password::Password};

pub struct Login {
    pub user_id: UserId,
    pub password: Password,
}

#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    #[error("no entry found")]
    EntryNotFound,

    #[error("failed to compare password and hash")]
    CheckHash,

    #[error("invalid password")]
    InvalidPassword,

    #[error("failed to generate random session id")]
    SessionGeneration,

    #[error("database error: {0}")]
    Database(#[from] database::Error),
}

impl Login {
    const MAX_SESSIONS: usize = 2;
    const SESSION_DURATION_HOURS: i64 = 24;

    pub async fn handle(
        self,
        tx: &mut database::Transaction<'_>,
    ) -> Result<auth_models::Session, LoginError> {
        let Some(entry) = database::queries::auth::get_entry_by_user_id(tx, &self.user_id).await?
        else {
            return Err(LoginError::EntryNotFound);
        };

        if !argon2::verify_password(
            self.password.value(),
            str::from_utf8(entry.hashed_password.as_ref()).expect("valid UTF-8"),
        )
        .map_err(|_| LoginError::CheckHash)?
        {
            return Err(LoginError::InvalidPassword);
        }

        if entry.sessions.len() >= Self::MAX_SESSIONS {
            tracing::debug!(
                user_id = %self.user_id,
                "too many sessions, oldest one will be deleted"
            );

            let session_to_delete = entry
                .oldest_session()
                .expect("entry has at least Self::MAX_SESSIONS sessions");
            database::queries::auth::delete_session(tx, &entry.id, &session_to_delete.id).await?;
        }

        let session = auth_models::Session {
            id: new_random_session_id()?,
            entry_id: entry.id,
            expires_at: Utc::now() + Duration::hours(Self::SESSION_DURATION_HOURS),
        };
        database::queries::auth::create_session(tx, &session).await?;

        Ok(session)
    }
}

fn new_random_session_id() -> Result<[u8; 128], LoginError> {
    let mut id = [0u8; 128];
    if OsRng.try_fill_bytes(&mut id).is_err() {
        return Err(LoginError::SessionGeneration);
    }
    Ok(id)
}
