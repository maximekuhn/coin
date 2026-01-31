use std::collections::{HashMap, HashSet};

use domain::{
    entities::{Expense, Group, User},
    types::user_id::UserId,
};

pub trait UserIdContainer {
    fn get_user_ids(&self) -> HashSet<UserId>;
}

pub async fn fetch_users(
    tx: &mut database::Transaction<'_>,
    containers: &[impl UserIdContainer],
) -> Result<HashMap<UserId, User>, database::Error> {
    let user_ids = containers.get_user_ids();
    database::queries::user::get_all_in_ids(tx, user_ids).await
}

impl<T: UserIdContainer> UserIdContainer for [T] {
    fn get_user_ids(&self) -> HashSet<UserId> {
        self.iter().flat_map(|item| item.get_user_ids()).collect()
    }
}

impl UserIdContainer for Group {
    fn get_user_ids(&self) -> HashSet<UserId> {
        let mut user_ids = HashSet::new();
        user_ids.insert(self.owner_id);
        self.members.iter().for_each(|member| {
            user_ids.insert(*member);
        });
        user_ids
    }
}

impl UserIdContainer for Expense {
    fn get_user_ids(&self) -> HashSet<UserId> {
        let mut user_ids = HashSet::new();
        user_ids.insert(self.payer_id);
        self.participants.iter().for_each(|participant| {
            user_ids.insert(*participant);
        });
        user_ids
    }
}
