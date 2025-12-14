use chrono::Utc;
use domain::{
    entities::User,
    types::{role::Role, user_id::UserId, username::Username},
};
use email_address::EmailAddress;

pub struct CreateUserCommand {
    pub email: EmailAddress,
    pub name: Username,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateUserError {
    #[error("email already taken")]
    EmailAlreadyTaken,

    #[error("database error: {0}")]
    Database(#[from] database::Error),
}

impl CreateUserCommand {
    pub async fn handle(
        self,
        tx: &mut database::Transaction<'_>,
    ) -> Result<UserId, CreateUserError> {
        if database::queries::user::email_exists(tx, &self.email).await? {
            return Err(CreateUserError::EmailAlreadyTaken);
        }

        let id = UserId::new_random();
        let created_at = Utc::now();
        let user = User::new(id, self.name, self.email, Role::User, created_at);

        database::queries::user::create(tx, &user).await?;

        Ok(id)
    }
}
