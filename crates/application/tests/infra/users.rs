use std::str::FromStr;

use application::commands::create_user::CreateUserCommand;
use domain::types::{role::Role, user_id::UserId};
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
        Ok(self
            .create_user_with_name_and_email(username, format!("{}@gmail.com", username).as_str())
            .await?)
    }

    pub async fn create_user_with_name_and_email(
        &mut self,
        username: &str,
        email: &str,
    ) -> anyhow::Result<Uuid> {
        let mut tx = self.pool.begin().await?;
        let name = username.parse()?;
        let email = EmailAddress::from_str(email)?;
        let user_id = CreateUserCommand { email, name }.handle(&mut tx).await?;
        tx.commit().await?;
        Ok(user_id.value())
    }

    pub async fn assert_user_exists(
        &mut self,
        user_id: Uuid,
        username: &str,
        email: &str,
        role: Role,
    ) -> anyhow::Result<()> {
        let mut tx = self.pool.begin().await?;
        let user = database::queries::user::get_by_id(&mut tx, &UserId::new(user_id)?).await?;
        assert!(user.is_some());
        let user = user.unwrap();
        assert_eq!(username, user.name.value());
        assert_eq!(email, user.email.email());
        assert_eq!(role, user.role);
        tx.commit().await?;
        Ok(())
    }
}
