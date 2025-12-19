use domain::types::{group_id::GroupId, user_id::UserId};

pub struct AddGroupMemberCommand {
    pub group_id: GroupId,
    pub user_id_to_add: UserId,
    pub current_user_id: UserId,
}

#[derive(Debug, thiserror::Error)]
pub enum AddGroupMemberError {
    #[error("only group owner can add member")]
    NotOwner,

    #[error("group not found")]
    GroupNotFound,

    #[error("user is already member of this group")]
    AlreadyMember,

    #[error("user to add not found")]
    UserNotFound,

    #[error("database error: {0}")]
    Database(#[from] database::Error),
}

impl AddGroupMemberCommand {
    pub async fn handle(
        self,
        tx: &mut database::Transaction<'_>,
    ) -> Result<(), AddGroupMemberError> {
        let Some(group) = database::queries::group::get_by_id(tx, &self.group_id).await? else {
            return Err(AddGroupMemberError::GroupNotFound);
        };

        if !database::queries::user::exists_by_id(tx, &self.user_id_to_add).await? {
            return Err(AddGroupMemberError::UserNotFound);
        }

        if !group.is_user_owner(&self.current_user_id) {
            return Err(AddGroupMemberError::NotOwner);
        }

        if group.is_user_owner(&self.user_id_to_add) || group.is_user_member(&self.user_id_to_add) {
            return Err(AddGroupMemberError::AlreadyMember);
        }

        database::queries::group::add_member(tx, &self.group_id, &self.user_id_to_add).await?;

        Ok(())
    }
}
