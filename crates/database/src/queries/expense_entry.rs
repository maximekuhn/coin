use domain::{
    entities::ExpenseEntry,
    types::{
        expense_entry_id::ExpenseEntryId, expense_entry_status::ExpenseEntryStatus,
        expense_id::ExpenseId,
    },
};
use sqlx::QueryBuilder;
use uuid::Uuid;

use crate::models::expense_entry::{
    DbExpenseEntry, DbExpenseEntryWithOptionalParticipant, DbExpenseEntryWithParticipants,
    flatten_expense_entries_with_participants,
};

pub async fn create(
    tx: &mut crate::Transaction<'_>,
    expense_entry: &domain::entities::ExpenseEntry,
) -> Result<(), crate::Error> {
    let expense_entry_id = expense_entry.id.value();
    sqlx::query(
        r#"
    INSERT INTO expense_entry 
    (id, expense_id, coin_group_id, payer_id, status, total, author_id, occurred_at, created_at)
    VALUES
    (?, ?, ?, ?, ?, ?, ?, ?, ?)
    "#,
    )
    .bind(expense_entry_id)
    .bind(expense_entry.expense_id.value())
    .bind(expense_entry.group_id.value())
    .bind(expense_entry.payer_id.value())
    .bind(match expense_entry.status {
        ExpenseEntryStatus::Active => None,
        ExpenseEntryStatus::Inactive { overwritten_by } => Some(overwritten_by.value()),
    })
    .bind(expense_entry.total.cents())
    .bind(expense_entry.author_id.value())
    .bind(expense_entry.occurred_at)
    .bind(expense_entry.created_at)
    .execute(tx.as_mut())
    .await?;

    if expense_entry.participants.is_empty() {
        return Ok(());
    }

    let mut qb = QueryBuilder::new(
        r#"
    INSERT INTO expense_entry_participant (expense_entry_id, participant_id)
    "#,
    );
    qb.push_values(&expense_entry.participants, |mut b, participant_id| {
        b.push_bind(expense_entry_id)
            .push_bind(participant_id.value());
    });
    qb.build().execute(tx.as_mut()).await?;

    Ok(())
}

pub async fn get_by_id(
    tx: &mut crate::Transaction<'_>,
    expense_entry_id: &ExpenseEntryId,
) -> Result<Option<ExpenseEntry>, crate::Error> {
    let expense_entry: Option<DbExpenseEntry> = sqlx::query_as(
        r#"
    SELECT
        id,
        expense_id,
        coin_group_id,
        payer_id,
        status,
        total,
        author_id,
        occurred_at,
        created_at
    FROM expense_entry
    WHERE id = ?
    "#,
    )
    .bind(expense_entry_id.value())
    .fetch_optional(tx.as_mut())
    .await?;

    let Some(expense_entry) = expense_entry else {
        return Ok(None);
    };

    let participants: Vec<(Uuid,)> = sqlx::query_as(
        r#"
    SELECT participant_id
    FROM expense_entry_participant
    WHERE expense_entry_id = ?
    "#,
    )
    .bind(expense_entry_id.value())
    .fetch_all(tx.as_mut())
    .await?;

    Ok(Some(
        DbExpenseEntryWithParticipants {
            entry: expense_entry,
            participants: participants.into_iter().map(|p| p.0).collect(),
        }
        .try_into()?,
    ))
}

pub async fn get_all_by_expense_id(
    tx: &mut crate::Transaction<'_>,
    expense_id: &ExpenseId,
) -> Result<Vec<ExpenseEntry>, crate::Error> {
    let rows: Vec<DbExpenseEntryWithOptionalParticipant> = sqlx::query_as(
        r#"
    SELECT
        ee.id,
        ee.expense_id,
        ee.coin_group_id,
        ee.payer_id,
        ee.status,
        ee.total,
        ee.author_id,
        ee.occurred_at,
        ee.created_at,
        eep.participant_id
    FROM expense_entry ee
    LEFT JOIN expense_entry_participant eep ON eep.expense_entry_id = ee.id
    WHERE ee.expense_id = ?
    "#,
    )
    .bind(expense_id.value())
    .fetch_all(tx.as_mut())
    .await?;

    flatten_expense_entries_with_participants(rows)
}
