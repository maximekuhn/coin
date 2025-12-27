use std::collections::{HashMap, HashSet};

use domain::{
    entities::Group,
    types::{group_id::GroupId, user_id::UserId},
};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct DbGroup {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
pub struct DbGroupMember {
    #[allow(unused)]
    #[sqlx(rename = "coin_group_id")]
    pub group_id: Uuid,
    pub member_id: Uuid,
}

pub struct DbGroupWithMembers {
    pub group: DbGroup,
    pub members: Vec<DbGroupMember>,
}

impl TryInto<Group> for DbGroupWithMembers {
    type Error = crate::Error;

    fn try_into(self) -> Result<Group, Self::Error> {
        let id = GroupId::new(self.group.id).map_err(|err| crate::Error::CorruptedData {
            msg: err.to_string(),
        })?;
        let groupname =
            self.group
                .name
                .parse()
                .map_err(
                    |err: domain::types::groupname::Error| crate::Error::CorruptedData {
                        msg: err.to_string(),
                    },
                )?;
        let owner_id =
            UserId::new(self.group.owner_id).map_err(|err| crate::Error::CorruptedData {
                msg: err.to_string(),
            })?;

        let mut group = Group::new(
            id,
            groupname,
            owner_id,
            HashSet::new(),
            self.group.created_at,
        );

        let members: Vec<UserId> = self
            .members
            .into_iter()
            .map(|member| UserId::new(member.member_id))
            .collect::<Result<_, _>>()
            .map_err(|err| crate::Error::CorruptedData {
                msg: err.to_string(),
            })?;
        group.members.extend(members);

        Ok(group)
    }
}

#[derive(sqlx::FromRow)]
pub struct DbGroupWithMember {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub member_id: Uuid,
    pub created_at: DateTime<Utc>,
}
pub fn flatten_group_with_member(rows: Vec<DbGroupWithMember>) -> Result<Vec<Group>, crate::Error> {
    let mut groups = Vec::new();

    let mut grs: HashMap<Uuid /* raw group id */, Vec<DbGroupWithMember>> = HashMap::new();
    for row in rows {
        grs.entry(row.id).or_default().push(row);
    }

    for gvs in grs.values() {
        let first = gvs.first().expect("first is here");
        let id = GroupId::new(first.id).map_err(|err| crate::Error::CorruptedData {
            msg: format!("corrupted group_id: {}", err),
        })?;
        let name = first
            .name
            .parse()
            .map_err(|err| crate::Error::CorruptedData {
                msg: format!("corrupted group_name: {}", err),
            })?;
        let owner_id = UserId::new(first.owner_id).map_err(|err| crate::Error::CorruptedData {
            msg: format!("corrupted owner_id: {}", err),
        })?;
        let member_id =
            UserId::new(first.member_id).map_err(|err| crate::Error::CorruptedData {
                msg: format!("corrupted member_id: {}", err),
            })?;
        let mut group = Group::new(
            id,
            name,
            owner_id,
            HashSet::from_iter(vec![member_id]),
            first.created_at,
        );

        for remaining in gvs.iter().skip(1) {
            let member_id =
                UserId::new(remaining.member_id).map_err(|err| crate::Error::CorruptedData {
                    msg: format!("corrupted member_id: {}", err),
                })?;
            group.members.insert(member_id);
        }

        groups.push(group);
    }

    Ok(groups)
}
