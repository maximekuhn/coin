use std::str::FromStr;

use application::commands::create_user::CreateUserCommand;
use email_address::EmailAddress;
use uuid::Uuid;

pub struct UsersHelper<'a> {
    pool: &'a database::SqlitePool,
}

impl<'a> UsersHelper<'a> {
    pub(super) fn new(pool: &'a database::SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&mut self, username: &str) -> anyhow::Result<Uuid> {
        let mut tx = self.pool.begin().await?;
        let name = username.parse()?;
        let email = EmailAddress::from_str(format!("{}@gmail.com", username).as_str())?;
        let user_id = CreateUserCommand { email, name }.handle(&mut tx).await?;
        tx.commit().await?;
        Ok(user_id.value())
    }
}
