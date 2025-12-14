use crate::extractors::user::User;

pub async fn hello_user(User(user, entry, _): User) -> String {
    tracing::info!(
        auth_sessions_count = entry.sessions.len(),
        "user called /api/hello"
    );
    format!("Hello, {}", user.name)
}
