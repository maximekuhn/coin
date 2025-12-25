use std::collections::HashSet;

use chrono::{DateTime, Utc};

use crate::types::{
    expense_entry_id::ExpenseEntryId, group_id::GroupId, money::Money, user_id::UserId,
};

#[derive(Debug, PartialEq)]
pub struct ExpenseEntry {
    pub id: ExpenseEntryId,
    pub group_id: GroupId,
    pub paid_by: UserId,
    pub participants: HashSet<UserId>,
    /// Total amount for the expense entry. Must be > 0.
    pub total: Money,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("total must be > 0")]
    NegativeTotal,
}

impl ExpenseEntry {
    pub fn new(
        id: ExpenseEntryId,
        group_id: GroupId,
        paid_by: UserId,
        participants: HashSet<UserId>,
        total: Money,
        created_at: DateTime<Utc>,
    ) -> Result<Self, Error> {
        if total.is_negative() {
            return Err(Error::NegativeTotal);
        }

        Ok(Self {
            id,
            group_id,
            paid_by,
            participants,
            total,
            created_at,
        })
    }
}
