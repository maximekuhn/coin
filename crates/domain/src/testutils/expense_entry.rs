use std::collections::HashSet;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    entities::ExpenseEntry,
    types::{expense_entry_status::ExpenseEntryStatus, money::Money, user_id::UserId},
};

pub struct TestExpenseEntry;

impl TestExpenseEntry {
    #[allow(clippy::too_many_arguments)]
    pub fn new_valid(
        id: impl Into<Uuid>,
        expense_id: impl Into<Uuid>,
        group_id: impl Into<Uuid>,
        payer_id: impl Into<Uuid>,
        participants: HashSet<impl Into<Uuid>>,
        status: ExpenseEntryStatus,
        total_euros: i64,
        author_id: impl Into<Uuid>,
        occurred_at: DateTime<Utc>,
        created_at: DateTime<Utc>,
    ) -> ExpenseEntry {
        let id = id.into().to_string().parse().unwrap();
        let expense_id = expense_id.into().to_string().parse().unwrap();
        let group_id = group_id.into().to_string().parse().unwrap();
        let payer_id = payer_id.into().to_string().parse().unwrap();
        let participants = participants
            .into_iter()
            .map(|p| UserId::new(p.into()).unwrap())
            .collect();
        let total = Money::from_euros(total_euros);
        let author_id = author_id.into().to_string().parse().unwrap();
        ExpenseEntry::new(
            id,
            expense_id,
            group_id,
            payer_id,
            participants,
            status,
            total,
            author_id,
            occurred_at,
            created_at,
        )
        .unwrap()
    }
}
