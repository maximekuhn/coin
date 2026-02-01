use std::num::NonZeroUsize;

use application::{
    commands::{
        add_group_member::AddGroupMemberCommand, create_empty_group::CreateEmptyGroupCommand,
    },
    pagination::Pagination,
    queries::{
        self, get_group_by_id::GetGroupByIdQuery, get_groups_for_user::GetGroupsForUserQuery,
    },
};
use domain::types::{
    group_id::GroupId,
    user_id::{self, UserId},
};
use uuid::Uuid;

pub struct GroupsHelper<'a> {
    pool: &'a database::SqlitePool,
}

impl<'a> GroupsHelper<'a> {
    pub(super) fn new(pool: &'a database::SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_empty_group(
        &mut self,
        groupname: &str,
        user_id: Uuid,
    ) -> anyhow::Result<Uuid> {
        let mut tx = self.pool.begin().await?;
        let group_id = CreateEmptyGroupCommand {
            groupname: groupname.parse()?,
            owner_id: UserId::new(user_id)?,
        }
        .handle(&mut tx)
        .await?;
        tx.commit().await?;
        Ok(group_id.value())
    }

    pub async fn add_member(
        &mut self,
        group_id: Uuid,
        owner_id: Uuid,
        user_to_add: Uuid,
    ) -> anyhow::Result<()> {
        let mut tx = self.pool.begin().await?;
        AddGroupMemberCommand {
            group_id: GroupId::new(group_id)?,
            user_id_to_add: UserId::new(user_to_add)?,
            current_user_id: UserId::new(owner_id)?,
        }
        .handle(&mut tx)
        .await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn get_all_for_user(
        &mut self,
        user_id: Uuid,
    ) -> anyhow::Result<queries::get_groups_for_user::Output> {
        let mut tx = self.pool.begin().await?;
        let output = GetGroupsForUserQuery {
            current_user: UserId::new(user_id)?,
            pagination: Pagination::new(
                NonZeroUsize::new(1).unwrap(),
                NonZeroUsize::new(1_000).unwrap(),
            )?,
        }
        .handle(&mut tx)
        .await?;
        tx.commit().await?;
        Ok(output)
    }

    pub async fn get_all_for_user_with_pagination(
        &mut self,
        user_id: Uuid,
        page: usize,
        page_size: usize,
    ) -> anyhow::Result<queries::get_groups_for_user::Output> {
        let mut tx = self.pool.begin().await?;
        let output = GetGroupsForUserQuery {
            current_user: UserId::new(user_id)?,
            pagination: Pagination::new(
                NonZeroUsize::new(page).unwrap(),
                NonZeroUsize::new(page_size).unwrap(),
            )?,
        }
        .handle(&mut tx)
        .await?;
        tx.commit().await?;
        Ok(output)
    }

    pub async fn get_group_by_id(
        &mut self,
        group_id: Uuid,
        user_id: Uuid,
    ) -> anyhow::Result<Option<domain::entities::Group>> {
        let mut tx = self.pool.begin().await?;
        let result = GetGroupByIdQuery {
            id: GroupId::new(group_id)?,
            current_user: UserId::new(user_id)?,
        }
        .handle(&mut tx)
        .await?;
        tx.commit().await?;
        Ok(result)
    }

    pub async fn assert_group_exists(
        &mut self,
        groupname: &str,
        user_id: Uuid,
    ) -> anyhow::Result<()> {
        let mut tx = self.pool.begin().await?;
        let exists = database::queries::group::exists_by_name_for_owner(
            &mut tx,
            &groupname.parse()?,
            &UserId::new(user_id)?,
        )
        .await?;
        tx.commit().await?;

        assert!(exists);

        Ok(())
    }

    pub async fn assert_group_contains_members(
        &mut self,
        group_id: Uuid,
        members: Vec<Uuid>,
    ) -> anyhow::Result<()> {
        let mut tx = self.pool.begin().await?;
        let group = database::queries::group::get_by_id(&mut tx, &GroupId::new(group_id)?).await?;
        tx.commit().await?;

        assert!(group.is_some(), "group not found");
        let group = group.unwrap();
        for member in members {
            assert!(group.is_user_member(&UserId::new(member)?));
        }
        Ok(())
    }
}
