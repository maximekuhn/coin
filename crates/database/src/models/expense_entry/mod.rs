use std::collections::HashSet;

use domain::{
    entities::ExpenseEntry,
    types::{
        expense_entry_id::ExpenseEntryId, expense_entry_status::ExpenseEntryStatus,
        expense_id::ExpenseId, group_id::GroupId, money::Money, user_id::UserId,
    },
};
use itertools::Itertools;
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct DbExpenseEntry {
    pub id: Uuid,
    pub expense_id: Uuid,
    #[sqlx(rename = "coin_group_id")]
    pub group_id: Uuid,
    pub payer_id: Uuid,
    /// empty = active, Some(expense_entry_id) = inactive
    pub status: Option<Uuid>,
    pub total: i64,
    pub author_id: Uuid,
    pub occurred_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

pub struct DbExpenseEntryWithParticipants {
    pub entry: DbExpenseEntry,
    pub participants: Vec<Uuid>,
}

impl TryInto<ExpenseEntry> for DbExpenseEntryWithParticipants {
    type Error = crate::Error;

    fn try_into(self) -> Result<ExpenseEntry, Self::Error> {
        let id = ExpenseEntryId::new(self.entry.id).map_err(|err| crate::Error::CorruptedData {
            msg: format!("corrupted id: {}", err),
        })?;
        let expense_id =
            ExpenseId::new(self.entry.expense_id).map_err(|err| crate::Error::CorruptedData {
                msg: format!("corrupted expense_id: {}", err),
            })?;
        let group_id =
            GroupId::new(self.entry.group_id).map_err(|err| crate::Error::CorruptedData {
                msg: format!("corrupted group_id: {}", err),
            })?;
        let payer_id =
            UserId::new(self.entry.payer_id).map_err(|err| crate::Error::CorruptedData {
                msg: format!("corrupted payer_id: {}", err),
            })?;
        let total = Money::from_cents(self.entry.total);
        let participants = self
            .participants
            .into_iter()
            .map(UserId::new)
            .collect::<Result<_, _>>()
            .map_err(|err| crate::Error::CorruptedData {
                msg: format!("corrupted participant(s): {}", err),
            })?;
        let status = match self.entry.status {
            Some(overwritten_by) => ExpenseEntryStatus::Inactive {
                overwritten_by: ExpenseEntryId::new(overwritten_by).map_err(|err| {
                    crate::Error::CorruptedData {
                        msg: format!("corrupted status: {}", err),
                    }
                })?,
            },
            None => ExpenseEntryStatus::Active,
        };
        let author_id =
            UserId::new(self.entry.author_id).map_err(|err| crate::Error::CorruptedData {
                msg: format!("corrupted author_id: {}", err),
            })?;

        ExpenseEntry::new(
            id,
            expense_id,
            group_id,
            payer_id,
            participants,
            status,
            total,
            author_id,
            self.entry.occurred_at,
            self.entry.created_at,
        )
        .map_err(|err| crate::Error::CorruptedData {
            msg: format!("corrupted expense entry: {}", err),
        })
    }
}

#[derive(sqlx::FromRow)]
pub struct DbExpenseEntryWithOptionalParticipant {
    pub id: Uuid,
    pub expense_id: Uuid,
    #[sqlx(rename = "coin_group_id")]
    pub group_id: Uuid,
    pub payer_id: Uuid,
    /// empty = active, Some(expense_entry_id) = inactive
    pub status: Option<Uuid>,
    pub total: i64,
    pub author_id: Uuid,
    pub occurred_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub participant_id: Option<Uuid>,
}

pub fn flatten_expense_entries_with_participants(
    rows: Vec<DbExpenseEntryWithOptionalParticipant>,
) -> Result<Vec<ExpenseEntry>, crate::Error> {
    let groupped_by_expense_id = rows.into_iter().into_group_map_by(|row| row.expense_id);
    let mut out = Vec::new();
    for entries in groupped_by_expense_id.into_values() {
        let first = entries.first().expect("no empty vector");

        let id = ExpenseEntryId::new(first.id).map_err(|err| crate::Error::CorruptedData {
            msg: format!("corrupted id: {}", err),
        })?;
        let expense_id =
            ExpenseId::new(first.expense_id).map_err(|err| crate::Error::CorruptedData {
                msg: format!("corrupted expense_id: {}", err),
            })?;
        let group_id = GroupId::new(first.group_id).map_err(|err| crate::Error::CorruptedData {
            msg: format!("corrupted group_id: {}", err),
        })?;
        let payer_id = UserId::new(first.payer_id).map_err(|err| crate::Error::CorruptedData {
            msg: format!("corrupted payer_id: {}", err),
        })?;
        let status = match first.status {
            Some(overwritten_by) => ExpenseEntryStatus::Inactive {
                overwritten_by: ExpenseEntryId::new(overwritten_by).map_err(|err| {
                    crate::Error::CorruptedData {
                        msg: format!("corrupted status: {}", err),
                    }
                })?,
            },
            None => ExpenseEntryStatus::Active,
        };
        let total = Money::from_cents(first.total);
        let author_id =
            UserId::new(first.author_id).map_err(|err| crate::Error::CorruptedData {
                msg: format!("corrupted author_id: {}", err),
            })?;

        let mut participants = HashSet::new();
        for entry in &entries {
            if let Some(participant) = entry.participant_id {
                participants.insert(UserId::new(participant).map_err(|err| {
                    crate::Error::CorruptedData {
                        msg: format!("corrupted participant: {}", err),
                    }
                })?);
            }
        }

        out.push(
            ExpenseEntry::new(
                id,
                expense_id,
                group_id,
                payer_id,
                participants,
                status,
                total,
                author_id,
                first.occurred_at,
                first.created_at,
            )
            .map_err(|err| crate::Error::CorruptedData {
                msg: format!("corrupted expense entry: {}", err),
            })?,
        );
    }
    Ok(out)
}
