use application::queries::get_user_by_id::{GetUserByIdError, GetUserByIdQuery};
use axum::extract::FromRequestParts;
use axum_extra::extract::CookieJar;

use crate::{
    auth::{authenticate::AuthenticateError, cookie},
    error::{ApiError, ErrorKind},
    state::AppState,
};

/// Extractor to retrieve information about the logged user.
pub struct User(
    /// Logged user.
    pub domain::entities::User,
    /// Auth entry.
    pub auth_models::Entry,
    /// Active session.
    pub auth_models::Session,
);

impl FromRequestParts<AppState> for User {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .expect("Infaillible");

        let Some(session_id) = cookie::get_valid_session_id_from_cookie(&jar) else {
            return Err(ApiError {
                kind: ErrorKind::InvalidCredentials,
                message: None,
                detail: Some(
                    "cookie does not exist or value is not a valid session id".to_string(),
                ),
            });
        };

        // Starts a transaction to retrieve information about the logged user.
        // This transaction is different than the one that will be used by the request,
        // which mean that data could not be the exact same once the request is processed
        // by the handler, but for authentication purposes this is acceptable.
        let mut tx = state.db_pool.begin().await?;

        let Some((entry, current_session)) = crate::auth::authenticate::Authenticate { session_id }
            .handle(&mut tx)
            .await
            .map_err(authenticate_err_to_api_error)?
        else {
            return Err(ApiError {
                kind: ErrorKind::InvalidCredentials,
                message: None,
                detail: Some("cookie found, but no associated entry or session exists".to_string()),
            });
        };

        let Some(user) = GetUserByIdQuery { id: entry.user_id }
            .handle(&mut tx)
            .await
            .map_err(get_user_by_id_err_to_api_error)?
        else {
            return Err(ApiError {
                kind: ErrorKind::Internal,
                message: None,
                detail: Some("session exists, but associated user could not be found".to_string()),
            });
        };

        tx.commit().await?;

        tracing::debug!(user_id = %user.id, "user successfully authenticated");

        Ok(User(user, entry, current_session))
    }
}

fn authenticate_err_to_api_error(err: AuthenticateError) -> ApiError {
    match err {
        AuthenticateError::Database(error) => ApiError {
            kind: ErrorKind::Internal,
            message: None,
            detail: Some(error.to_string()),
        },
        AuthenticateError::ExpiredSession => ApiError {
            kind: ErrorKind::SessionExpired,
            message: None,
            detail: None,
        },
    }
}

fn get_user_by_id_err_to_api_error(err: GetUserByIdError) -> ApiError {
    match err {
        GetUserByIdError::Database(error) => ApiError {
            kind: ErrorKind::Internal,
            message: None,
            detail: Some(error.to_string()),
        },
    }
}
