use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use domain::{
    entities::{Expense, ExpenseEntry, Group},
    types::{expense_id::ExpenseId, group_id::GroupId, money::Money, user_id::UserId},
};

use crate::{
    common,
    models::{group::GroupSummary, user::UserSummary},
    pagination::Pagination,
};

pub struct GetLatestExpensesForUserQuery {
    pub current_user: UserId,
    pub pagination: Pagination,
}

#[derive(Debug, thiserror::Error)]
pub enum GetLatestExpensesForUserError {
    #[error("database error: {0}")]
    Database(#[from] database::Error),
}

impl GetLatestExpensesForUserQuery {
    pub async fn handle(
        self,
        tx: &mut database::Transaction<'_>,
    ) -> Result<Output, GetLatestExpensesForUserError> {
        let expenses: Vec<Expense> = database::queries::expense_entry::get_all_active_for_user(
            tx,
            &self.current_user,
            self.pagination.into(),
        )
        .await?
        .into_iter()
        .map(ExpenseEntry::into)
        .collect();

        let total_count =
            database::queries::expense_entry::count_all_active_for_user(tx, &self.current_user)
                .await?;

        let group_ids: HashSet<GroupId> = expenses.iter().map(|e| e.group_id).collect();

        let mut groups: HashMap<GroupId, Group> = HashMap::new();
        for group_id in group_ids {
            let group = database::queries::group::get_by_id(tx, &group_id)
                .await?
                .expect("corrupted data: cannot have expense associated to a non-existing group");
            groups.insert(group_id, group);
        }

        let users = common::user::fetch_users(tx, &expenses).await?;

        let mut expense_summaries = Vec::with_capacity(expenses.len());
        for expense in expenses {
            let group = groups
                .get(&expense.group_id)
                .expect("corrupted data: group must be here");

            let payer = users
                .get(&expense.payer_id)
                .expect("corrupted data: payer id must be here");

            let expense_summary = ExpenseSummary {
                id: expense.id,
                group: GroupSummary {
                    id: group.id,
                    name: group.name.clone(),
                },
                occurred_at: expense.occurred_at,
                total: expense.total,
                paid_by: UserSummary {
                    id: payer.id,
                    name: payer.name.clone(),
                },
            };

            expense_summaries.push(expense_summary);
        }

        Ok(Output {
            expenses: expense_summaries,
            total_items: total_count as usize,
        })
    }
}

#[derive(Debug)]
pub struct Output {
    pub expenses: Vec<ExpenseSummary>,
    pub total_items: usize,
}

#[derive(Debug, PartialEq)]
pub struct ExpenseSummary {
    pub id: ExpenseId,
    pub group: GroupSummary,
    pub occurred_at: DateTime<Utc>,
    pub total: Money,
    pub paid_by: UserSummary,
}
