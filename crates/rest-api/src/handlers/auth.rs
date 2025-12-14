use application::{
    commands::create_user::{CreateUserCommand, CreateUserError},
    queries::get_user_by_email::{GetUserByEmailError, GetUserByEmailQuery},
};
use axum::{Json, extract::State};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    auth::{cookie, login::LoginError, logout::LogoutError, register::RegisterError},
    error::{ApiError, ErrorKind},
    extractors::user::User,
    state::AppState,
};

pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<RegisterBody>,
) -> Result<Json<RegisterResponse>, ApiError> {
    let name = body.username.parse()?;
    let email = body.email.parse()?;
    let password = body.password.parse()?;

    let mut tx = state.db_pool.begin().await?;

    let user_id = CreateUserCommand { email, name }
        .handle(&mut tx)
        .await
        .map_err(create_user_err_to_api_error)?;

    let entry = crate::auth::register::Register { user_id, password }
        .handle(&mut tx)
        .await
        .map_err(register_err_to_api_error)?;

    tracing::info!(%user_id, entry_id = %entry.id, "user registered successfully");

    tx.commit().await?;

    Ok(Json(RegisterResponse {
        user_id: user_id.value(),
    }))
}

pub async fn login(
    jar: CookieJar,
    State(state): State<AppState>,
    Json(body): Json<LoginBody>,
) -> Result<CookieJar, ApiError> {
    let email = body.email.parse()?;
    let password = body.password.parse()?;

    let mut tx = state.db_pool.begin().await?;

    let Some(user) = GetUserByEmailQuery { email }
        .handle(&mut tx)
        .await
        .map_err(get_user_by_email_err_to_api_error)?
    else {
        return Err(ApiError {
            kind: ErrorKind::InvalidCredentials,
            message: None,
            detail: Some("user not found by email".to_string()),
        });
    };

    let session = crate::auth::login::Login {
        user_id: user.id,
        password,
    }
    .handle(&mut tx)
    .await
    .map_err(login_err_to_api_error)?;

    tx.commit().await?;

    let env = match state.config.domain {
        Some(domain) => cookie::Environment::Prod { domain },
        None => cookie::Environment::Dev,
    };
    let cookie = cookie::generate_cookie(&session, env);
    Ok(jar.add(cookie))
}

pub async fn logout(
    jar: CookieJar,
    User(_, _, session): User,
    State(state): State<AppState>,
) -> Result<CookieJar, ApiError> {
    let mut tx = state.db_pool.begin().await?;

    crate::auth::logout::Logout { session }
        .handle(&mut tx)
        .await
        .map_err(logout_err_to_api_error)?;

    tx.commit().await?;

    Ok(cookie::remove_cookie(jar))
}

#[derive(Deserialize)]
pub struct RegisterBody {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterResponse {
    user_id: Uuid,
}

#[derive(Deserialize)]
pub struct LoginBody {
    email: String,
    password: String,
}

fn create_user_err_to_api_error(err: CreateUserError) -> ApiError {
    match err {
        CreateUserError::EmailAlreadyTaken => ApiError {
            kind: ErrorKind::Conflict,
            message: Some("email already taken".to_string()),
            detail: None,
        },
        CreateUserError::Database(error) => ApiError {
            kind: ErrorKind::Internal,
            message: None,
            detail: Some(error.to_string()),
        },
    }
}

fn register_err_to_api_error(err: RegisterError) -> ApiError {
    match err {
        RegisterError::AlreadyRegistered => ApiError {
            kind: ErrorKind::Conflict,
            message: Some("user already registered".to_string()),
            detail: None,
        },
        RegisterError::Hash => ApiError {
            kind: ErrorKind::Internal,
            message: None,
            detail: Some("failed to hash password".to_string()),
        },
        RegisterError::Database(error) => ApiError {
            kind: ErrorKind::Internal,
            message: None,
            detail: Some(error.to_string()),
        },
    }
}

fn get_user_by_email_err_to_api_error(err: GetUserByEmailError) -> ApiError {
    match err {
        GetUserByEmailError::Database(error) => ApiError {
            kind: ErrorKind::Internal,
            message: None,
            detail: Some(error.to_string()),
        },
    }
}

fn login_err_to_api_error(err: LoginError) -> ApiError {
    let (kind, message, detail) = match err {
        LoginError::EntryNotFound => (
            ErrorKind::InvalidCredentials,
            None,
            Some("entry not found".to_string()),
        ),
        LoginError::CheckHash => (ErrorKind::Internal, None, Some(err.to_string())),
        LoginError::InvalidPassword => (
            ErrorKind::InvalidCredentials,
            None,
            Some("invalid password".to_string()),
        ),
        LoginError::SessionGeneration => (ErrorKind::Internal, None, Some(err.to_string())),
        LoginError::Database(error) => (ErrorKind::Internal, None, Some(error.to_string())),
    };
    ApiError {
        kind,
        message,
        detail,
    }
}

fn logout_err_to_api_error(err: LogoutError) -> ApiError {
    match err {
        LogoutError::SessionNotFound => ApiError {
            kind: ErrorKind::Internal,
            message: None,
            detail: Some("session not found (found in User extractor)".to_string()),
        },
        LogoutError::Database(error) => ApiError {
            kind: ErrorKind::Internal,
            message: None,
            detail: Some(error.to_string()),
        },
    }
}
