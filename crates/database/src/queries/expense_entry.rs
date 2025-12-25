use domain::{entities::ExpenseEntry, types::expense_entry_id::ExpenseEntryId};
use sqlx::QueryBuilder;
use uuid::Uuid;

use crate::models::expense_entry::{DbExpenseEntry, DbExpenseEntryWithParticipants};

pub async fn create(
    tx: &mut crate::Transaction<'_>,
    expense_entry: &domain::entities::ExpenseEntry,
) -> Result<(), crate::Error> {
    let expense_entry_id = expense_entry.id.value();
    sqlx::query(
        r#"
    INSERT INTO expense_entry (id, coin_group_id, payer_id, total, created_at) VALUES
    (?, ?, ?, ?, ?)
    "#,
    )
    .bind(expense_entry_id)
    .bind(expense_entry.group_id.value())
    .bind(expense_entry.paid_by.value())
    .bind(expense_entry.total.cents())
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
        coin_group_id,
        payer_id,
        total,
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
    SELECT  participant_id
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
