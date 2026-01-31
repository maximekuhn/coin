use domain::types::{group_id::GroupId, groupname::Groupname};

#[derive(Debug)]
pub struct GroupSummary {
    pub id: GroupId,
    pub name: Groupname,
}
