use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    entities::Group,
    types::{group_id::GroupId, user_id::UserId},
};

pub struct TestGroup;

impl TestGroup {
    pub fn new_valid(
        id: impl Into<Uuid>,
        name: impl Into<String>,
        owner_id: impl Into<Uuid>,
        members: Vec<impl Into<Uuid>>,
        created_at: DateTime<Utc>,
    ) -> Group {
        let id = GroupId::new(id.into()).unwrap();
        let name = name.into().parse().unwrap();
        let owner_id = UserId::new(owner_id.into()).unwrap();
        let members = members
            .into_iter()
            .map(|member_id| UserId::new(member_id.into()).unwrap())
            .collect();
        Group::new(id, name, owner_id, members, created_at)
    }
}
