use std::collections::HashSet;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    entities::ExpenseEntry,
    types::{expense_entry_id::ExpenseEntryId, group_id::GroupId, money::Money, user_id::UserId},
};

pub struct TestExpenseEntry;

impl TestExpenseEntry {
    pub fn new_valid(
        id: impl Into<Uuid>,
        group_id: impl Into<Uuid>,
        paid_by: impl Into<Uuid>,
        participants: HashSet<impl Into<Uuid>>,
        total_euros: i64,
        created_at: DateTime<Utc>,
    ) -> ExpenseEntry {
        ExpenseEntry::new(
            ExpenseEntryId::new(id.into()).unwrap(),
            GroupId::new(group_id.into()).unwrap(),
            UserId::new(paid_by.into()).unwrap(),
            participants
                .into_iter()
                .map(|participant_id| UserId::new(participant_id.into()).unwrap())
                .collect(),
            Money::from_euros(total_euros),
            created_at,
        )
        .unwrap()
    }
}
