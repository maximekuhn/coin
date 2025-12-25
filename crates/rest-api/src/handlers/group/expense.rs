use application::commands::create_expense::{
    CreateExpenseCommand, CreateExpenseError, IncludeParticipants,
};
use axum::{
    Json,
    extract::{Path, State},
};
use chrono::{DateTime, Utc};
use domain::types::{group_id::GroupId, money::Money, user_id::UserId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    error::{ApiError, ErrorKind},
    extractors::user::User,
    state::AppState,
};

pub async fn create(
    State(state): State<AppState>,
    User(user, _, _): User,
    Path(group_id): Path<Uuid>,
    Json(body): Json<CreateBody>,
) -> Result<Json<CreateResponse>, ApiError> {
    let group_id = GroupId::new(group_id)?;
    let total = Money::from_euros(body.total_euros as i64);
    let participants = match body.participants {
        Some(participants) => IncludeParticipants::List {
            participants: participants
                .into_iter()
                .map(UserId::new)
                .collect::<Result<_, _>>()?,
        },
        None => IncludeParticipants::All,
    };
    let payer_id = UserId::new(body.payer_id)?;

    let mut tx = state.db_pool.begin().await?;

    let expense_id = CreateExpenseCommand {
        group_id,
        payer_id,
        participants,
        total,
        author_id: user.id,
        occured_at: body.occured_at,
    }
    .handle(&mut tx)
    .await
    .map_err(create_expense_err_to_api_error)?;

    tx.commit().await?;

    Ok(Json(CreateResponse {
        expense_id: expense_id.value(),
    }))
}

#[derive(Deserialize)]
pub struct CreateBody {
    /// If participants is `None`, all group members will be considered
    /// as participants for the new expense.
    participants: Option<Vec<Uuid>>,
    total_euros: u64,
    occured_at: DateTime<Utc>,
    payer_id: Uuid,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateResponse {
    expense_id: Uuid,
}

fn create_expense_err_to_api_error(err: CreateExpenseError) -> ApiError {
    match err {
        CreateExpenseError::GroupNotFound => ApiError {
            kind: ErrorKind::NotFound,
            message: Some("group not found".to_string()),
            detail: None,
        },
        CreateExpenseError::InvalidTotal => ApiError {
            kind: ErrorKind::InvalidInput,
            message: Some("total_euros must be a positive amount".to_string()),
            detail: None,
        },
        CreateExpenseError::PayerIsNotGroupMember => ApiError {
            kind: ErrorKind::InvalidInput,
            message: Some("payer does not exist in the group".to_string()),
            detail: None,
        },
        CreateExpenseError::AuthorNotInGroup => ApiError {
            kind: ErrorKind::ActionForbidden,
            message: None,
            detail: Some("current user is not in group".to_string()),
        },
        CreateExpenseError::ParticipantNotFound => ApiError {
            kind: ErrorKind::NotFound,
            message: Some(
                "at least one participant in the list does not belong to the group".to_string(),
            ),
            detail: None,
        },
        CreateExpenseError::Database(error) => ApiError {
            kind: ErrorKind::Internal,
            message: None,
            detail: Some(error.to_string()),
        },
    }
}
