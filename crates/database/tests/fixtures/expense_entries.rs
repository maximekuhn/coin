use std::collections::HashSet;

use domain::testutils::expense_entry::TestExpenseEntry;
use sqlx::types::chrono::DateTime;
use uuid::Uuid;

pub fn john_and_bill_shared_shared_expenses_expense_1() -> domain::entities::ExpenseEntry {
    TestExpenseEntry::new_valid(
        Uuid::parse_str("019b5551261d768c80a3d047d742e141").unwrap(),
        super::groups::john_and_bill_shared_expenses().id.value(),
        super::users::bill().id.value(),
        HashSet::from_iter(vec![super::users::johndoe().id.value()]),
        10,
        DateTime::parse_from_rfc3339("2025-12-02T10:00:50Z")
            .unwrap()
            .to_utc(),
    )
}
