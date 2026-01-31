use std::num::NonZeroUsize;

use application::{
    commands::{
        add_group_member::{AddGroupMemberCommand, AddGroupMemberError},
        create_empty_group::{CreateEmptyGroupCommand, CreateEmptyGroupError},
    },
    pagination::Pagination,
    queries::get_groups_for_user::{GetGroupsForUserError, GetGroupsForUserQuery},
};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use domain::types::{group_id::GroupId, user_id::UserId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    dtos::list_response::ListResponse,
    error::{ApiError, ErrorKind},
    extractors::user::User,
    state::AppState,
};

pub mod expense;

pub async fn create(
    State(state): State<AppState>,
    User(user, _, _): User,
    Json(body): Json<CreateBody>,
) -> Result<Json<CreateResponse>, ApiError> {
    let groupname = body.name.parse()?;

    let mut tx = state.db_pool.begin().await?;

    let group_id = CreateEmptyGroupCommand {
        groupname,
        owner_id: user.id,
    }
    .handle(&mut tx)
    .await
    .map_err(create_empty_group_err_to_api_error)?;

    tx.commit().await?;

    Ok(Json(CreateResponse {
        group_id: group_id.value(),
    }))
}

pub async fn add_member(
    State(state): State<AppState>,
    User(user, _, _): User,
    Path(group_id): Path<Uuid>,
    Json(body): Json<AddMemberBody>,
) -> Result<StatusCode, ApiError> {
    let group_id = GroupId::new(group_id)?;
    let new_member_id = UserId::new(body.user_id)?;
    let current_user_id = user.id;

    let mut tx = state.db_pool.begin().await?;

    AddGroupMemberCommand {
        group_id,
        user_id_to_add: new_member_id,
        current_user_id,
    }
    .handle(&mut tx)
    .await
    .map_err(add_member_err_to_api_error)?;

    tx.commit().await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_all_overview(
    State(state): State<AppState>,
    User(user, _, _): User,
    Query(query): Query<GetAllQuery>,
) -> Result<Json<ListResponse<GroupDto>>, ApiError> {
    let pagination = Pagination::new_from_optional(query.page, query.page_size)?;

    let mut tx = state.db_pool.begin().await?;

    let output = GetGroupsForUserQuery {
        current_user: user.id,
        pagination,
    }
    .handle(&mut tx)
    .await
    .map_err(get_groups_for_user_err_to_api_error)?;

    tx.commit().await?;

    tracing::debug!(
        total = output.total_items,
        returned = output.groups.len(),
        "query output summary"
    );

    let groups = output.groups.into_iter().map(GroupDto::from).collect();
    Ok(Json(ListResponse {
        data: groups,
        request_pagination: pagination.into(),
        total_items: output.total_items,
    }))
}

#[derive(Deserialize)]
pub struct CreateBody {
    name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateResponse {
    group_id: Uuid,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMemberBody {
    user_id: Uuid,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllQuery {
    pub page: Option<NonZeroUsize>,
    pub page_size: Option<NonZeroUsize>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupDto {
    id: Uuid,
    name: String,
    owner: UserDto,
    created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_activity: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    current_user_balance_euros: Option<i64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UserDto {
    id: Uuid,
    name: String,
}

impl From<application::queries::get_groups_for_user::GroupSummary> for GroupDto {
    fn from(group_summary: application::queries::get_groups_for_user::GroupSummary) -> Self {
        Self {
            id: group_summary.id.value(),
            name: group_summary.name.value(),
            owner: group_summary.owner.into(),
            created_at: group_summary.created_at,
            last_activity: group_summary
                .last_expense
                .map(|expense| expense.occurred_at),
            current_user_balance_euros: group_summary
                .current_user_balance
                .map(|balance| balance.euros()),
        }
    }
}

impl From<application::queries::get_groups_for_user::UserSummary> for UserDto {
    fn from(user_summary: application::queries::get_groups_for_user::UserSummary) -> Self {
        Self {
            id: user_summary.id.value(),
            name: user_summary.name.value(),
        }
    }
}

fn create_empty_group_err_to_api_error(err: CreateEmptyGroupError) -> ApiError {
    match err {
        CreateEmptyGroupError::NameNotAvailable => ApiError {
            kind: ErrorKind::Conflict,
            message: Some(
                "another group with the same name for the same owner already exists".to_string(),
            ),
            detail: None,
        },
        CreateEmptyGroupError::OwnerNotFound => ApiError {
            kind: ErrorKind::Internal,
            message: None,
            detail: Some(
                "owner is logged in, but create empty group could not find it".to_string(),
            ),
        },
        CreateEmptyGroupError::Database(error) => ApiError {
            kind: ErrorKind::Internal,
            message: None,
            detail: Some(error.to_string()),
        },
    }
}

fn add_member_err_to_api_error(err: AddGroupMemberError) -> ApiError {
    match err {
        AddGroupMemberError::NotOwner => ApiError {
            kind: ErrorKind::ActionForbidden,
            message: None,
            detail: None,
        },
        AddGroupMemberError::GroupNotFound => ApiError {
            kind: ErrorKind::NotFound,
            message: Some("group not found".to_string()),
            detail: None,
        },
        AddGroupMemberError::AlreadyMember => ApiError {
            kind: ErrorKind::Conflict,
            message: Some(err.to_string()),
            detail: None,
        },
        AddGroupMemberError::Database(error) => ApiError {
            kind: ErrorKind::Internal,
            message: None,
            detail: Some(error.to_string()),
        },
        AddGroupMemberError::UserNotFound => ApiError {
            kind: ErrorKind::NotFound,
            message: Some("user to add not found".to_string()),
            detail: None,
        },
    }
}

fn get_groups_for_user_err_to_api_error(err: GetGroupsForUserError) -> ApiError {
    match err {
        GetGroupsForUserError::Database(error) => ApiError {
            kind: ErrorKind::Internal,
            message: None,
            detail: Some(error.to_string()),
        },
    }
}
