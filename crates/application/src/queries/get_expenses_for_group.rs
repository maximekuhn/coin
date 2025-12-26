use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use domain::{
    entities::{ExpenseEntry, User},
    types::{
        expense_id::ExpenseId, group_id::GroupId, money::Money, user_id::UserId, username::Username,
    },
};

use crate::pagination::Pagination;

pub struct GetExpensesForGroupQuery {
    pub group_id: GroupId,
    pub current_user: UserId,
    pub pagination: Pagination,
}

#[derive(Debug, thiserror::Error)]
pub enum GetExpensesForGroupError {
    #[error("group not found")]
    GroupNotFound,

    #[error("forbidden")]
    Forbidden,

    #[error("database error: {0}")]
    Database(#[from] database::Error),
}

impl GetExpensesForGroupQuery {
    pub async fn handle(
        self,
        tx: &mut database::Transaction<'_>,
    ) -> Result<Output, GetExpensesForGroupError> {
        let Some(group) = database::queries::group::get_by_id(tx, &self.group_id).await? else {
            return Err(GetExpensesForGroupError::GroupNotFound);
        };

        if !group.contains_user(&self.current_user) {
            return Err(GetExpensesForGroupError::Forbidden);
        }

        let expense_entries = database::queries::expense_entry::get_all_active_for_group(
            tx,
            &self.group_id,
            self.pagination.into(),
        )
        .await?;

        let total_expense_entries =
            database::queries::expense_entry::count_all_active_for_group(tx, &self.group_id)
                .await?;

        let user_ids = get_user_ids(&expense_entries);
        let users = database::queries::user::get_all_in_ids(tx, user_ids).await?;

        Ok(Output {
            expenses: build_group_expenses(expense_entries, users),
            total_items: total_expense_entries as usize,
        })
    }
}

fn get_user_ids(expense_entries: &[ExpenseEntry]) -> HashSet<UserId> {
    let mut ids = HashSet::new();
    for expense_entry in expense_entries {
        ids.insert(expense_entry.payer_id);
        for participant in &expense_entry.participants {
            ids.insert(*participant);
        }
    }
    ids
}

fn build_group_expenses(
    expense_entries: Vec<ExpenseEntry>,
    users: HashMap<UserId, User>,
) -> Vec<GroupExpense> {
    let mut expenses = Vec::new();
    for expense_entry in expense_entries {
        let payer = users
            .get(&expense_entry.payer_id)
            .expect("corrupted data: payer is not here")
            .clone();
        expenses.push(GroupExpense {
            id: expense_entry.expense_id,
            payer: UserSummary {
                id: payer.id,
                name: payer.name,
            },
            participants: get_participants(&expense_entry, &users),
            total: expense_entry.total,
            occurred_at: expense_entry.occurred_at,
        });
    }
    expenses
}

fn get_participants(
    expense_entry: &ExpenseEntry,
    users: &HashMap<UserId, User>,
) -> Vec<UserSummary> {
    let mut participants = Vec::new();
    for participant in &expense_entry.participants {
        let user = users
            .get(participant)
            .expect("corrupted data: participant is not here")
            .clone();
        participants.push(UserSummary {
            id: user.id,
            name: user.name,
        });
    }
    participants
}

pub struct Output {
    pub expenses: Vec<GroupExpense>,
    pub total_items: usize,
}

pub struct GroupExpense {
    pub id: ExpenseId,
    pub payer: UserSummary,
    pub participants: Vec<UserSummary>,
    pub total: Money,
    pub occurred_at: DateTime<Utc>,
}

pub struct UserSummary {
    pub id: UserId,
    pub name: Username,
}
