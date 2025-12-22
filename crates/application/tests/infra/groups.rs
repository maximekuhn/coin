use application::commands::{
    add_group_member::AddGroupMemberCommand, create_empty_group::CreateEmptyGroupCommand,
};
use domain::types::{group_id::GroupId, user_id::UserId};
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
