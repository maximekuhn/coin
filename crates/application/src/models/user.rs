use domain::types::{user_id::UserId, username::Username};

#[derive(Debug, PartialEq)]
pub struct UserSummary {
    pub id: UserId,
    pub name: Username,
}
