use std::collections::HashSet;

use chrono::{DateTime, Utc};

use crate::types::{group_id::GroupId, groupname::Groupname, user_id::UserId};

#[derive(derive_new::new, Debug, PartialEq)]
pub struct Group {
    pub id: GroupId,
    pub name: Groupname,
    pub owner_id: UserId,
    pub members: HashSet<UserId>,
    pub created_at: DateTime<Utc>,
}

impl Group {
    pub fn is_user_owner(&self, user_id: &UserId) -> bool {
        self.owner_id == *user_id
    }

    pub fn is_user_member(&self, user_id: &UserId) -> bool {
        self.members.iter().any(|member_id| member_id == user_id)
    }

    pub fn contains_user(&self, user_id: &UserId) -> bool {
        self.is_user_owner(user_id) || self.is_user_member(user_id)
    }
}
