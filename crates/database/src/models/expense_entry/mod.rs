use domain::{
    entities::ExpenseEntry,
    types::{expense_entry_id::ExpenseEntryId, group_id::GroupId, money::Money, user_id::UserId},
};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct DbExpenseEntry {
    pub id: Uuid,
    #[sqlx(rename = "coin_group_id")]
    pub group_id: Uuid,
    pub payer_id: Uuid,
    pub total: i64,
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
        let group_id =
            GroupId::new(self.entry.group_id).map_err(|err| crate::Error::CorruptedData {
                msg: format!("corrupted group_id: {}", err),
            })?;
        let paid_by =
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

        ExpenseEntry::new(
            id,
            group_id,
            paid_by,
            participants,
            total,
            self.entry.created_at,
        )
        .map_err(|err| crate::Error::CorruptedData {
            msg: format!("corrupted expense entry: {}", err),
        })
    }
}
