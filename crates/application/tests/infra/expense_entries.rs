use application::commands::create_expense::{CreateExpenseCommand, IncludeParticipants};
use domain::{
    entities::ExpenseEntry,
    types::{expense_entry_id::ExpenseEntryId, group_id::GroupId, money::Money, user_id::UserId},
};
use uuid::Uuid;

pub struct ExpenseEntriesHelper<'a> {
    pool: &'a database::SqlitePool,
}

impl<'a> ExpenseEntriesHelper<'a> {
    pub(super) fn new(pool: &'a database::SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_expense_for_all_group_members(
        &mut self,
        group_id: Uuid,
        payer_id: Uuid,
        total_euros: i64,
    ) -> anyhow::Result<Uuid> {
        let mut tx = self.pool.begin().await?;
        let id = CreateExpenseCommand {
            group_id: GroupId::new(group_id)?,
            payer_id: UserId::new(payer_id)?,
            participants: IncludeParticipants::All,
            total: Money::from_euros(total_euros),
        }
        .handle(&mut tx)
        .await?;
        tx.commit().await?;
        Ok(id.value())
    }

    pub async fn create_expense(
        &mut self,
        group_id: Uuid,
        payer_id: Uuid,
        total_euros: i64,
        participants: Vec<Uuid>,
    ) -> anyhow::Result<Uuid> {
        let mut tx = self.pool.begin().await?;
        let id = CreateExpenseCommand {
            group_id: GroupId::new(group_id)?,
            payer_id: UserId::new(payer_id)?,
            participants: IncludeParticipants::List {
                participants: participants
                    .into_iter()
                    .map(|id| UserId::new(id))
                    .collect::<Result<_, _>>()?,
            },
            total: Money::from_euros(total_euros),
        }
        .handle(&mut tx)
        .await?;
        tx.commit().await?;
        Ok(id.value())
    }

    pub async fn assert_entry_exists(
        &mut self,
        expense_entry_id: Uuid,
    ) -> anyhow::Result<ExpenseEntry> {
        let mut tx = self.pool.begin().await?;
        let expense_entry = database::queries::expense_entry::get_by_id(
            &mut tx,
            &ExpenseEntryId::new(expense_entry_id)?,
        )
        .await?;
        tx.commit().await?;
        assert!(expense_entry.is_some());
        Ok(expense_entry.unwrap())
    }
}
