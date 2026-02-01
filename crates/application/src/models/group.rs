use domain::types::{group_id::GroupId, groupname::Groupname};

#[derive(Debug, PartialEq)]
pub struct GroupSummary {
    pub id: GroupId,
    pub name: Groupname,
}
