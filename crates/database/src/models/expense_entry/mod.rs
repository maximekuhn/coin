use std::collections::HashSet;

use domain::{
    entities::ExpenseEntry,
    types::{
        expense_entry_id::ExpenseEntryId, expense_entry_status::ExpenseEntryStatus,
        expense_id::ExpenseId, group_id::GroupId, money::Money, user_id::UserId,
    },
};
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
    let mut out = Vec::new();
    let mut current: Option<(DbExpenseEntryWithOptionalParticipant, HashSet<UserId>)> = None;

    for row in rows {
        match &mut current {
            Some((first, participants)) if first.id == row.id => {
                if let Some(participant_id) = row.participant_id {
                    participants.insert(UserId::new(participant_id).map_err(|err| {
                        crate::Error::CorruptedData {
                            msg: format!("corrupted participant: {}", err),
                        }
                    })?);
                }
            }
            _ => {
                if let Some((first, participants)) = current.take() {
                    out.push(build_expense_entry(first, participants)?);
                }

                let mut participants = HashSet::new();
                if let Some(participant_id) = row.participant_id {
                    participants.insert(UserId::new(participant_id).map_err(|err| {
                        crate::Error::CorruptedData {
                            msg: format!("corrupted participant: {}", err),
                        }
                    })?);
                }

                current = Some((row, participants));
            }
        }
    }

    if let Some((first, participants)) = current {
        out.push(build_expense_entry(first, participants)?);
    }

    Ok(out)
}

fn build_expense_entry(
    first: DbExpenseEntryWithOptionalParticipant,
    participants: HashSet<UserId>,
) -> Result<ExpenseEntry, crate::Error> {
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

    let author_id = UserId::new(first.author_id).map_err(|err| crate::Error::CorruptedData {
        msg: format!("corrupted author_id: {}", err),
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
    })
}
