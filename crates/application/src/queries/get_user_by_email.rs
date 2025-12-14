use domain::entities::User;
use email_address::EmailAddress;

pub struct GetUserByEmailQuery {
    pub email: EmailAddress,
}

#[derive(Debug, thiserror::Error)]
pub enum GetUserByEmailError {
    #[error("database error: {0}")]
    Database(#[from] database::Error),
}

impl GetUserByEmailQuery {
    pub async fn handle(
        self,
        tx: &mut database::Transaction<'_>,
    ) -> Result<Option<User>, GetUserByEmailError> {
        Ok(database::queries::user::get_by_email(tx, &self.email).await?)
    }
}
