use domain::{
    entities::Group,
    types::{group_id::GroupId, user_id::UserId},
};

pub struct GetGroupByIdQuery {
    pub id: GroupId,
    pub current_user: UserId,
}

#[derive(Debug, thiserror::Error)]
pub enum GetGroupByIdError {
    #[error("current user does not belong to the requested group")]
    CurrentUserNotInGroup,

    #[error("database error: {0}")]
    Database(#[from] database::Error),
}

impl GetGroupByIdQuery {
    pub async fn handle(
        self,
        tx: &mut database::Transaction<'_>,
    ) -> Result<Option<Group>, GetGroupByIdError> {
        let Some(group) = database::queries::group::get_by_id(tx, &self.id).await? else {
            return Ok(None);
        };

        if !group.contains_user(&self.current_user) {
            return Err(GetGroupByIdError::CurrentUserNotInGroup);
        }

        Ok(Some(group))
    }
}
