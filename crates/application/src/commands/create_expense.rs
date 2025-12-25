use std::collections::HashSet;

use chrono::Utc;
use domain::{
    entities::{ExpenseEntry, Group},
    types::{expense_entry_id::ExpenseEntryId, group_id::GroupId, money::Money, user_id::UserId},
};

pub struct CreateExpenseCommand {
    pub group_id: GroupId,
    pub payer_id: UserId,
    pub participants: IncludeParticipants,
    pub total: Money,
}

pub enum IncludeParticipants {
    /// All current group members.
    All,

    /// List of participants; each of them must be a group member.
    List { participants: HashSet<UserId> },
}

#[derive(Debug, thiserror::Error)]
pub enum CreateExpenseError {
    #[error("group not found")]
    GroupNotFound,

    #[error("cannot create an expense with a negative total")]
    NegativeTotal,

    #[error("must be group member to add an expense to a group")]
    PayerIsNotGroupMember,

    #[error("at least one participant is not found in group")]
    ParticipantNotFound,

    #[error("database error: {0}")]
    Database(#[from] database::Error),
}

impl CreateExpenseCommand {
    pub async fn handle(
        self,
        tx: &mut database::Transaction<'_>,
    ) -> Result<ExpenseEntryId, CreateExpenseError> {
        if self.total.is_negative() {
            return Err(CreateExpenseError::NegativeTotal);
        }

        let Some(group) = database::queries::group::get_by_id(tx, &self.group_id).await? else {
            return Err(CreateExpenseError::GroupNotFound);
        };

        if !(group.is_user_owner(&self.payer_id) || group.is_user_member(&self.payer_id)) {
            return Err(CreateExpenseError::PayerIsNotGroupMember);
        }

        let mut participants = self.get_participants(&group)?;
        participants.remove(&self.payer_id);

        let expense_entry = ExpenseEntry::new(
            ExpenseEntryId::new_random(),
            self.group_id,
            self.payer_id,
            participants,
            self.total,
            Utc::now(),
        )
        .expect("valid expense entry");
        database::queries::expense_entry::create(tx, &expense_entry).await?;
        Ok(expense_entry.id)
    }

    fn get_participants(&self, group: &Group) -> Result<HashSet<UserId>, CreateExpenseError> {
        match &self.participants {
            IncludeParticipants::All => {
                let mut participants = HashSet::new();
                participants.insert(group.owner_id);
                group.members.iter().for_each(|p| {
                    participants.insert(*p);
                });
                Ok(participants)
            }
            IncludeParticipants::List { participants } => {
                if !all_participants_in_group(group, participants) {
                    return Err(CreateExpenseError::ParticipantNotFound);
                }
                Ok(participants.clone())
            }
        }
    }
}

fn all_participants_in_group(group: &Group, participants: &HashSet<UserId>) -> bool {
    let mut members: HashSet<UserId> = HashSet::from_iter(group.members.clone());
    members.insert(group.owner_id);
    members.is_superset(participants)
}
