use application::commands::{
    add_group_member::{AddGroupMemberCommand, AddGroupMemberError},
    create_empty_group::{CreateEmptyGroupCommand, CreateEmptyGroupError},
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use domain::types::{group_id::GroupId, user_id::UserId};
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
