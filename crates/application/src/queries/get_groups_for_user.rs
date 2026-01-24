use std::{
    collections::{HashMap, HashSet},
    num::NonZeroUsize,
};

use chrono::{DateTime, Utc};
use domain::{
    entities::{Expense, ExpenseEntry, Group, User},
    services::user_balance::compute_user_balance,
    types::{
        group_id::GroupId, groupname::Groupname, money::Money, user_id::UserId, username::Username,
    },
};

use crate::pagination::Pagination;

pub struct GetGroupsForUserQuery {
    pub current_user: UserId,
    pub pagination: Pagination,
}

#[derive(Debug, thiserror::Error)]
pub enum GetGroupsForUserError {
    #[error("database error: {0}")]
    Database(#[from] database::Error),
}

impl GetGroupsForUserQuery {
    pub async fn handle(
        self,
        tx: &mut database::Transaction<'_>,
    ) -> Result<Output, GetGroupsForUserError> {
        let groups = database::queries::group::get_all_for_user(
            tx,
            &self.current_user,
            self.pagination.into(),
        )
        .await?;

        let total_groups =
            database::queries::group::count_all_for_user(tx, &self.current_user).await?;

        if groups.is_empty() {
            return Ok(Output {
                groups: vec![],
                total_items: total_groups as usize,
            });
        }

        let user_ids = get_user_ids(&groups);
        let users = database::queries::user::get_all_in_ids(tx, user_ids).await?;

        let groups_with_expenses = get_all_expenses_for_groups(tx, groups).await?;

        Ok(Output {
            groups: build_group_summaries(groups_with_expenses, users, self.current_user),
            total_items: total_groups as usize,
        })
    }
}

async fn get_all_expenses_for_groups(
    tx: &mut database::Transaction<'_>,
    groups: Vec<Group>,
) -> Result<Vec<(Group, Vec<Expense>)>, database::Error> {
    let mut out = Vec::new();
    for group in groups {
        let expenses = get_all_expenses_for_group(tx, &group).await?;
        out.push((group, expenses));
    }
    Ok(out)
}

async fn get_all_expenses_for_group(
    tx: &mut database::Transaction<'_>,
    group: &Group,
) -> Result<Vec<Expense>, database::Error> {
    let total = database::queries::expense_entry::count_all_active_for_group(tx, &group.id).await?;

    let mut out = Vec::with_capacity(total as usize);
    let mut current_page = NonZeroUsize::new(1).expect("1 is a valid NonZeroUsize");
    let page_size = Pagination::default().page_size();

    while (out.len() as u64) < total {
        let pagination = Pagination::new(current_page, page_size).expect("valid pagination");
        let page = database::queries::expense_entry::get_all_active_for_group(
            tx,
            &group.id,
            pagination.into(),
        )
        .await?;

        if page.is_empty() {
            break;
        }

        out.extend(page.into_iter().map(ExpenseEntry::into));
        current_page = current_page.checked_add(1).expect("valid NonZeroUsize");
    }

    Ok(out)
}

fn build_group_summaries(
    groups_with_expenses: Vec<(Group, Vec<Expense>)>,
    users: HashMap<UserId, User>,
    current_user: UserId,
) -> Vec<GroupSummary> {
    let mut out = Vec::new();
    for (group, expenses) in groups_with_expenses {
        let owner = users
            .get(&group.owner_id)
            .expect("corrupted data: missing group owner");
        let owner_summary = UserSummary {
            id: owner.id,
            name: owner.name.clone(),
        };
        let current_user_balance = compute_user_balance(&expenses, current_user);
        let last_expense = expenses
            .into_iter()
            .max_by_key(|expense| expense.occurred_at);
        let gs = GroupSummary {
            id: group.id,
            name: group.name,
            owner: owner_summary,
            created_at: group.created_at,
            last_expense,
            current_user_balance,
        };
        out.push(gs);
    }
    out
}

fn get_user_ids(groups: &[Group]) -> HashSet<UserId> {
    let mut ids = HashSet::new();
    for group in groups {
        ids.insert(group.owner_id);
        for member in &group.members {
            ids.insert(*member);
        }
    }
    ids
}

pub struct Output {
    pub groups: Vec<GroupSummary>,
    pub total_items: usize,
}

pub struct GroupSummary {
    pub id: GroupId,
    pub name: Groupname,
    pub owner: UserSummary,
    pub created_at: DateTime<Utc>,
    pub last_expense: Option<Expense>,
    pub current_user_balance: Option<Money>,
}

pub struct UserSummary {
    pub id: UserId,
    pub name: Username,
}
