use std::collections::HashSet;

use chrono::{DateTime, Utc};

use crate::{
    entities::ExpenseEntry,
    types::{expense_id::ExpenseId, group_id::GroupId, money::Money, user_id::UserId},
};

#[derive(Debug, PartialEq)]
pub struct Expense {
    // Expense identifier.
    pub id: ExpenseId,

    /// Group to which this expense belongs.
    pub group_id: GroupId,

    /// User who actually paid the expense in real life.
    pub payer_id: UserId,

    /// Users participating in this expense.
    pub participants: HashSet<UserId>,

    /// Total amount of the expense.
    /// Must be strictly greater than zero.
    pub total: Money,

    /// Real-world time at which the expense occurred.
    /// This may differ from created_at if the expense is entered later.
    pub occurred_at: DateTime<Utc>,

    /// System time at which this expense was created in the system.
    pub created_at: DateTime<Utc>,
}

impl Expense {
    pub fn contains_user(&self, user_id: &UserId) -> bool {
        self.is_payer(user_id) || self.participants.contains(user_id)
    }

    pub fn is_payer(&self, user_id: &UserId) -> bool {
        self.payer_id == *user_id
    }
}

impl From<ExpenseEntry> for Expense {
    fn from(expense_entry: ExpenseEntry) -> Self {
        Self {
            id: expense_entry.expense_id,
            group_id: expense_entry.group_id,
            payer_id: expense_entry.payer_id,
            participants: expense_entry.participants,
            total: expense_entry.total,
            occurred_at: expense_entry.occurred_at,
            created_at: expense_entry.created_at,
        }
    }
}
