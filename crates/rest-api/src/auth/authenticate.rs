use chrono::Utc;

pub struct Authenticate {
    pub session_id: [u8; 128],
}

#[derive(Debug, thiserror::Error)]
pub enum AuthenticateError {
    #[error("session expired")]
    ExpiredSession,

    #[error("database error: {0}")]
    Database(#[from] database::Error),
}

impl Authenticate {
    pub async fn handle(
        self,
        tx: &mut database::Transaction<'_>,
    ) -> Result<Option<(auth_models::Entry, auth_models::Session)>, AuthenticateError> {
        let Some(session) = database::queries::auth::get_session_by_id(tx, self.session_id).await?
        else {
            return Ok(None);
        };

        if session.is_expired(Utc::now()) {
            return Err(AuthenticateError::ExpiredSession);
        }

        let entry = database::queries::auth::get_entry_by_id(tx, &session.entry_id)
            .await?
            .expect("session does not exist without entry");

        Ok(Some((entry, session)))
    }
}
