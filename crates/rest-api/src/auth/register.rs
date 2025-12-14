use chrono::Utc;
use domain::types::user_id::UserId;
use uuid::Uuid;

use crate::auth::{argon2, password::Password};

pub struct Register {
    pub user_id: UserId,
    pub password: Password,
}

#[derive(Debug, thiserror::Error)]
pub enum RegisterError {
    #[error("user already registered")]
    AlreadyRegistered,

    #[error("failed to hash password")]
    Hash,

    #[error("database error: {0}")]
    Database(#[from] database::Error),
}

impl Register {
    pub async fn handle(
        self,
        tx: &mut database::Transaction<'_>,
    ) -> Result<auth_models::Entry, RegisterError> {
        if database::queries::auth::entry_exists_for_user_id(tx, &self.user_id).await? {
            return Err(RegisterError::AlreadyRegistered);
        }

        let hashed_password =
            argon2::hash_password(self.password).map_err(|_| RegisterError::Hash)?;

        let entry = auth_models::Entry {
            id: Uuid::now_v7(),
            user_id: self.user_id,
            hashed_password,
            created_at: Utc::now(),
            sessions: Vec::new(),
        };

        database::queries::auth::create_entry(tx, &entry).await?;

        Ok(entry)
    }
}
