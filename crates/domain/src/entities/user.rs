use chrono::{DateTime, Utc};
use email_address::EmailAddress;

use crate::types::{role::Role, user_id::UserId, username::Username};

#[derive(derive_new::new, Debug, PartialEq)]
pub struct User {
    pub id: UserId,
    pub name: Username,
    pub email: EmailAddress,
    pub role: Role,
    pub created_at: DateTime<Utc>,
}
