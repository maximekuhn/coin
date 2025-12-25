use application::commands::create_expense::{CreateExpenseCommand, IncludeParticipants};
use chrono::{DateTime, Utc};
use domain::{
    entities::ExpenseEntry,
    types::{
        expense_entry_id::ExpenseEntryId, expense_id::ExpenseId, group_id::GroupId, money::Money,
        user_id::UserId,
    },
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
        author_id: Uuid,
        occured_at: DateTime<Utc>,
    ) -> anyhow::Result<Uuid> {
        let mut tx = self.pool.begin().await?;
        let id = CreateExpenseCommand {
            group_id: GroupId::new(group_id)?,
            payer_id: UserId::new(payer_id)?,
            participants: IncludeParticipants::All,
            total: Money::from_euros(total_euros),
            author_id: UserId::new(author_id)?,
            occured_at,
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
        author_id: Uuid,
        occured_at: DateTime<Utc>,
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
            author_id: UserId::new(author_id)?,
            occured_at,
        }
        .handle(&mut tx)
        .await?;
        tx.commit().await?;
        Ok(id.value())
    }

    pub async fn assert_expense_has_a_single_entry(
        &mut self,
        expense_id: Uuid,
    ) -> anyhow::Result<ExpenseEntry> {
        let mut tx = self.pool.begin().await?;
        let expense_entries = database::queries::expense_entry::get_all_by_expense_id(
            &mut tx,
            &ExpenseId::new(expense_id)?,
        )
        .await?;
        tx.commit().await?;
        assert_eq!(1, expense_entries.len());
        Ok(expense_entries.into_iter().nth(0).unwrap())
    }
}
