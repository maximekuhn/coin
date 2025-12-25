use crate::types::expense_entry_id::ExpenseEntryId;

#[derive(Debug, PartialEq)]
pub enum ExpenseEntryStatus {
    Active,
    Inactive { overwritten_by: ExpenseEntryId },
}
