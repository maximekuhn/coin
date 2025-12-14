pub struct Logout {
    pub session: auth_models::Session,
}

#[derive(Debug, thiserror::Error)]
pub enum LogoutError {
    #[error("session not found")]
    SessionNotFound,

    #[error("database error: {0}")]
    Database(#[from] database::Error),
}

impl Logout {
    pub async fn handle(self, tx: &mut database::Transaction<'_>) -> Result<(), LogoutError> {
        let Some(session) = database::queries::auth::get_session_by_id(tx, self.session.id).await?
        else {
            return Err(LogoutError::SessionNotFound);
        };
        database::queries::auth::delete_session(tx, &session.entry_id, &session.id).await?;
        Ok(())
    }
}
