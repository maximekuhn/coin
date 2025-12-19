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

        let mut group = Group::new(id, groupname, owner_id, vec![], self.group.created_at);

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
