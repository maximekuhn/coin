use application::commands::create_empty_group::CreateEmptyGroupCommand;
use domain::types::user_id::UserId;
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
}
