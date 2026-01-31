use std::num::NonZeroUsize;

use application::{
    pagination::Pagination,
    queries::get_latest_expenses_for_user::{
        GetLatestExpensesForUserError, GetLatestExpensesForUserQuery,
    },
};
use axum::{
    Json,
    extract::{Query, State},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    dtos::{list_response::ListResponse, user::MinimalUserDto},
    error::{ApiError, ErrorKind},
    extractors::user::User,
    state::AppState,
};

pub async fn get_latest_for_user(
    State(state): State<AppState>,
    User(user, _, _): User,
    Query(query): Query<GetLatestQuery>,
) -> Result<Json<ListResponse<ExpenseDto>>, ApiError> {
    let pagination = Pagination::new_from_optional(query.page, query.page_size)?;

    let mut tx = state.db_pool.begin().await?;

    let output = GetLatestExpensesForUserQuery {
        current_user: user.id,
        pagination,
    }
    .handle(&mut tx)
    .await
    .map_err(get_latest_expenses_for_user_err_to_api_error)?;

    tx.commit().await?;

    let expenses = output.expenses.into_iter().map(ExpenseDto::from).collect();
    Ok(Json(ListResponse {
        data: expenses,
        request_pagination: pagination.into(),
        total_items: output.total_items,
    }))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLatestQuery {
    pub page: Option<NonZeroUsize>,
    pub page_size: Option<NonZeroUsize>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpenseDto {
    id: Uuid,
    group: GroupDto,
    occurred_at: DateTime<Utc>,
    total_euros: i64,
    paid_by: MinimalUserDto,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GroupDto {
    id: Uuid,
    name: String,
}

impl From<application::queries::get_latest_expenses_for_user::ExpenseSummary> for ExpenseDto {
    fn from(es: application::queries::get_latest_expenses_for_user::ExpenseSummary) -> Self {
        Self {
            id: es.id.value(),
            group: es.group.into(),
            occurred_at: es.occurred_at,
            total_euros: es.total.euros(),
            paid_by: es.paid_by.into(),
        }
    }
}

impl From<application::models::group::GroupSummary> for GroupDto {
    fn from(gs: application::models::group::GroupSummary) -> Self {
        Self {
            id: gs.id.value(),
            name: gs.name.value(),
        }
    }
}

fn get_latest_expenses_for_user_err_to_api_error(err: GetLatestExpensesForUserError) -> ApiError {
    match err {
        GetLatestExpensesForUserError::Database(error) => ApiError {
            kind: ErrorKind::Internal,
            message: None,
            detail: Some(error.to_string()),
        },
    }
}
