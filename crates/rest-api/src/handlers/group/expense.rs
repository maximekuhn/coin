use application::{
    commands::create_expense::{CreateExpenseCommand, CreateExpenseError, IncludeParticipants},
    queries::get_expenses_for_group::{GetExpensesForGroupError, GetExpensesForGroupQuery},
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
        occured_at: body.occurred_at,
    }
    .handle(&mut tx)
    .await
    .map_err(create_expense_err_to_api_error)?;

    tx.commit().await?;

    Ok(Json(CreateResponse {
        expense_id: expense_id.value(),
    }))
}

pub async fn get_all(
    State(state): State<AppState>,
    User(user, _, _): User,
    Path(group_id): Path<Uuid>,
) -> Result<Json<GetAllResponse>, ApiError> {
    let group_id = GroupId::new(group_id)?;

    let mut tx = state.db_pool.begin().await?;

    let expenses = GetExpensesForGroupQuery {
        group_id,
        current_user: user.id,
    }
    .handle(&mut tx)
    .await
    .map_err(get_group_expenses_err_to_api_error)?;

    tx.commit().await?;

    tracing::debug!(total = expenses.len(), "found expenses");

    let expenses = expenses.into_iter().map(ExpenseDto::from).collect();
    Ok(Json(GetAllResponse { expenses }))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBody {
    /// If participants is `None`, all group members will be considered
    /// as participants for the new expense.
    participants: Option<Vec<Uuid>>,
    total_euros: u64,
    occurred_at: DateTime<Utc>,
    payer_id: Uuid,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateResponse {
    expense_id: Uuid,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllResponse {
    expenses: Vec<ExpenseDto>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ExpenseDto {
    id: Uuid,
    payer: UserDto,
    participants: Vec<UserDto>,
    total_euros: i64,
    occured_at: DateTime<Utc>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UserDto {
    id: Uuid,
    name: String,
}

impl From<application::queries::get_expenses_for_group::GroupExpense> for ExpenseDto {
    fn from(group_expense: application::queries::get_expenses_for_group::GroupExpense) -> Self {
        Self {
            id: group_expense.id.value(),
            payer: group_expense.payer.into(),
            participants: group_expense
                .participants
                .into_iter()
                .map(UserDto::from)
                .collect(),
            total_euros: group_expense.total.euros(),
            occured_at: group_expense.occurred_at,
        }
    }
}

impl From<application::queries::get_expenses_for_group::UserSummary> for UserDto {
    fn from(user_summary: application::queries::get_expenses_for_group::UserSummary) -> Self {
        Self {
            id: user_summary.id.value(),
            name: user_summary.name.value(),
        }
    }
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
            kind: ErrorKind::InvalidInput,
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

fn get_group_expenses_err_to_api_error(err: GetExpensesForGroupError) -> ApiError {
    match err {
        GetExpensesForGroupError::GroupNotFound => ApiError {
            kind: ErrorKind::NotFound,
            message: Some("group not found".to_string()),
            detail: None,
        },
        GetExpensesForGroupError::Forbidden => ApiError {
            kind: ErrorKind::ActionForbidden,
            message: None,
            detail: Some("user is not allowed access group expenses".to_string()),
        },
        GetExpensesForGroupError::Database(error) => ApiError {
            kind: ErrorKind::Internal,
            message: None,
            detail: Some(error.to_string()),
        },
    }
}
