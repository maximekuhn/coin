use std::collections::HashSet;

use domain::{
    testutils::expense_entry::TestExpenseEntry, types::expense_entry_status::ExpenseEntryStatus,
};
use sqlx::types::chrono::DateTime;
use uuid::Uuid;

pub fn john_and_bill_shared_shared_expenses_active_expense_entry() -> domain::entities::ExpenseEntry
{
    TestExpenseEntry::new_valid(
        Uuid::parse_str("019b5551261d768c80a3d047d742e141").unwrap(),
        Uuid::parse_str("019b5648dcf47d7a8fbb2a414de4bcc6").unwrap(),
        super::groups::john_and_bill_shared_expenses().id.value(),
        super::users::bill().id.value(),
        HashSet::from_iter(vec![super::users::johndoe().id.value()]),
        ExpenseEntryStatus::Active,
        10,
        super::users::bill().id.value(),
        DateTime::parse_from_rfc3339("2025-12-01T08:00:50Z")
            .unwrap()
            .to_utc(),
        DateTime::parse_from_rfc3339("2025-12-02T10:00:50Z")
            .unwrap()
            .to_utc(),
    )
}

pub fn john_and_bill_shared_shared_expenses_overwritten_expense_entry()
-> domain::entities::ExpenseEntry {
    TestExpenseEntry::new_valid(
        Uuid::parse_str("019b5652e5ed7776b05b2a326da8471a").unwrap(),
        Uuid::parse_str("019b5648dcf47d7a8fbb2a414de4bcc6").unwrap(),
        super::groups::john_and_bill_shared_expenses().id.value(),
        super::users::bill().id.value(),
        HashSet::from_iter(vec![super::users::johndoe().id.value()]),
        ExpenseEntryStatus::Inactive {
            overwritten_by: john_and_bill_shared_shared_expenses_active_expense_entry().id,
        },
        8,
        super::users::johndoe().id.value(),
        DateTime::parse_from_rfc3339("2025-12-01T08:00:50Z")
            .unwrap()
            .to_utc(),
        DateTime::parse_from_rfc3339("2025-12-01T16:00:50Z")
            .unwrap()
            .to_utc(),
    )
}
