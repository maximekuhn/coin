use chrono::Utc;
use domain::{
    entities::Group,
    types::{group_id::GroupId, groupname::Groupname, user_id::UserId},
};

pub struct CreateEmptyGroupCommand {
    pub groupname: Groupname,
    pub owner_id: UserId,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateEmptyGroupError {
    #[error("another group for owner with the same name already exists")]
    NameNotAvailable,

    #[error("specified owner does not exist")]
    OwnerNotFound,

    #[error("database error: {0}")]
    Database(#[from] database::Error),
}

impl CreateEmptyGroupCommand {
    pub async fn handle(
        self,
        tx: &mut database::Transaction<'_>,
    ) -> Result<GroupId, CreateEmptyGroupError> {
        if !database::queries::user::exists_by_id(tx, &self.owner_id).await? {
            return Err(CreateEmptyGroupError::OwnerNotFound);
        }

        if database::queries::group::exists_by_name_for_owner(tx, &self.groupname, &self.owner_id)
            .await?
        {
            return Err(CreateEmptyGroupError::NameNotAvailable);
        }

        let id = GroupId::new_random();
        let group = Group::new(id, self.groupname, self.owner_id, vec![], Utc::now());
        database::queries::group::create(tx, &group).await?;

        Ok(id)
    }
}
