use application::commands::create_empty_group::{CreateEmptyGroupCommand, CreateEmptyGroupError};
use axum::{Json, extract::State};
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

#[derive(Deserialize)]
pub struct CreateBody {
    name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateResponse {
    group_id: Uuid,
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
