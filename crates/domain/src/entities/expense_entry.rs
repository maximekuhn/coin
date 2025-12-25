use std::collections::HashSet;

use chrono::{DateTime, Utc};

use crate::types::{
    expense_entry_id::ExpenseEntryId, expense_entry_status::ExpenseEntryStatus,
    expense_id::ExpenseId, group_id::GroupId, money::Money, user_id::UserId,
};

/// Represents a versioned snapshot of an [Expense](crate::entities::expense::Expense).
///
/// Expense entries are immutable.
/// Any modification to an expense (correction, update, etc.)
/// results in the creation of a new ExpenseEntry with a new `id`,
/// while sharing the same `expense_id`.
///
/// At any time, exactly one ExpenseEntry per `expense_id` must be
/// marked as Active. Older versions are kept for audit and history.
#[derive(Debug, PartialEq)]
pub struct ExpenseEntry {
    /// Unique identifier for this specific version of the expense.
    /// Each edit on an Expense creates a new ExpenseEntry with a new id.
    pub id: ExpenseEntryId,

    /// Logical identifier of the expense.
    /// All versions of the same expense share the same expense_id.
    pub expense_id: ExpenseId,

    /// Group to which this expense belongs.
    pub group_id: GroupId,

    /// User who actually paid the expense in real life.
    pub payer_id: UserId,

    /// Users participating in this expense.
    pub participants: HashSet<UserId>,

    /// Status of this expense entry.
    /// Exactly one entry per expense_id must be Active at any time.
    pub status: ExpenseEntryStatus,

    /// Total amount of the expense.
    /// Must be strictly greater than zero.
    pub total: Money,

    /// User who created this version of the expense entry.
    /// Used for audit trail and permission checks.
    pub author_id: UserId,

    /// Real-world time at which the expense occurred.
    /// This may differ from created_at if the expense is entered later.
    pub occurred_at: DateTime<Utc>,

    /// System time at which this expense entry was created in the system.
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("total must be > 0")]
    NegativeTotal,
}

impl ExpenseEntry {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: ExpenseEntryId,
        expense_id: ExpenseId,
        group_id: GroupId,
        payer_id: UserId,
        participants: HashSet<UserId>,
        status: ExpenseEntryStatus,
        total: Money,
        author_id: UserId,
        occurred_at: DateTime<Utc>,
        created_at: DateTime<Utc>,
    ) -> Result<Self, Error> {
        if total.is_negative() {
            return Err(Error::NegativeTotal);
        }

        Ok(Self {
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
        })
    }
}
