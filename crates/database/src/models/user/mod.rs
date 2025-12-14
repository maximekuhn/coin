use domain::{entities::User, types::user_id::UserId};
use sqlx::{
    prelude::FromRow,
    types::chrono::{DateTime, Utc},
};
use uuid::Uuid;

use crate::models::user::db_role::DbRole;

pub mod db_role;

#[derive(FromRow)]
pub struct DbUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: u8,
    pub created_at: DateTime<Utc>,
}

impl TryInto<User> for DbUser {
    type Error = crate::Error;

    fn try_into(self) -> Result<User, Self::Error> {
        let id = UserId::new(self.id).map_err(to_corrupted_data_err)?;
        let name = self.name.parse().map_err(to_corrupted_data_err)?;
        let email = self.email.parse().map_err(to_corrupted_data_err)?;
        let role = DbRole(self.role).try_into()?;
        Ok(User {
            id,
            name,
            email,
            role,
            created_at: self.created_at,
        })
    }
}

fn to_corrupted_data_err<E>(err: E) -> crate::Error
where
    E: std::error::Error,
{
    crate::Error::CorruptedData {
        msg: format!("corrupted data: {}", err),
    }
}
