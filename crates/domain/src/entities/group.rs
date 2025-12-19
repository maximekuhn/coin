use chrono::{DateTime, Utc};

use crate::types::{group_id::GroupId, groupname::Groupname, user_id::UserId};

#[derive(derive_new::new)]
pub struct Group {
    pub id: GroupId,
    pub name: Groupname,
    pub owner_id: UserId,
    pub members: Vec<UserId>,
    pub created_at: DateTime<Utc>,
}
