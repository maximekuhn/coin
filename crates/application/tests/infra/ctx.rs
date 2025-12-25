use crate::infra::{
    expense_entries::ExpenseEntriesHelper, groups::GroupsHelper, users::UsersHelper,
};

pub struct TestContext {
    pool: database::SqlitePool,
}

impl TestContext {
    pub fn new(pool: database::SqlitePool) -> Self {
        Self { pool }
    }

    pub fn users(&self) -> UsersHelper<'_> {
        UsersHelper::new(&self.pool)
    }

    pub fn groups(&self) -> GroupsHelper<'_> {
        GroupsHelper::new(&self.pool)
    }

    pub fn expense_entries(&self) -> ExpenseEntriesHelper<'_> {
        ExpenseEntriesHelper::new(&self.pool)
    }
}
