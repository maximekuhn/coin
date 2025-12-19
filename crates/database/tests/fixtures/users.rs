use domain::{testutils::user::TestUser, types::role::Role};
use sqlx::types::chrono::DateTime;
use uuid::Uuid;

pub fn johndoe() -> domain::entities::User {
    TestUser::new_valid(
        Uuid::parse_str("019b14ef290a70d9a2452a4723d9d44a").unwrap(),
        "johndoe",
        "john.doe@gmail.com",
        Role::User,
        DateTime::parse_from_rfc3339("2025-01-01T12:00:00Z")
            .unwrap()
            .to_utc(),
    )
}

pub fn bill() -> domain::entities::User {
    TestUser::new_valid(
        Uuid::parse_str("019b3752b7d87a208bb28d0a44a1f661").unwrap(),
        "bill",
        "bill@gmail.com",
        Role::User,
        DateTime::parse_from_rfc3339("2025-03-01T16:30:00Z")
            .unwrap()
            .to_utc(),
    )
}

pub fn marc() -> domain::entities::User {
    TestUser::new_valid(
        Uuid::parse_str("019b375cdc4f757aa3423df76fc97f40").unwrap(),
        "marc",
        "marc@gmail.com",
        Role::User,
        DateTime::parse_from_rfc3339("2025-03-03T23:59:59Z")
            .unwrap()
            .to_utc(),
    )
}
