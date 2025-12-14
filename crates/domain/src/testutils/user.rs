use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    entities::User,
    types::{role::Role, user_id::UserId},
};

pub struct TestUser;

impl TestUser {
    pub fn new_valid(
        id: impl Into<Uuid>,
        name: impl Into<String>,
        email: impl Into<String>,
        role: Role,
        created_at: DateTime<Utc>,
    ) -> User {
        let id = UserId::new(id.into()).unwrap();
        let name = name.into().parse().unwrap();
        let email = email.into().parse().unwrap();
        User::new(id, name, email, role, created_at)
    }
}
