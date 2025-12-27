use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use domain::{
    entities::{Group, User},
    types::{group_id::GroupId, groupname::Groupname, user_id::UserId, username::Username},
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

        Ok(Output {
            groups: build_group_summaries(groups, users),
            total_items: total_groups as usize,
        })
    }
}

fn build_group_summaries(groups: Vec<Group>, users: HashMap<UserId, User>) -> Vec<GroupSummary> {
    let mut out = Vec::new();
    for group in groups {
        let owner = users
            .get(&group.owner_id)
            .expect("corrupted data: missing group owner");
        let owner_summary = UserSummary {
            id: owner.id,
            name: owner.name.clone(),
        };
        let gs = GroupSummary {
            id: group.id,
            name: group.name,
            owner: owner_summary,
            created_at: group.created_at,
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
}

pub struct UserSummary {
    pub id: UserId,
    pub name: Username,
}
