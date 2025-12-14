use domain::{entities::User, types::user_id::UserId};

pub struct GetUserByIdQuery {
    pub id: UserId,
}

#[derive(Debug, thiserror::Error)]
pub enum GetUserByIdError {
    #[error("database error: {0}")]
    Database(#[from] database::Error),
}

impl GetUserByIdQuery {
    pub async fn handle(
        self,
        tx: &mut database::Transaction<'_>,
    ) -> Result<Option<User>, GetUserByIdError> {
        Ok(database::queries::user::get_by_id(tx, &self.id).await?)
    }
}
