use domain::testutils::group::TestGroup;
use sqlx::types::chrono::DateTime;
use uuid::Uuid;

pub fn trip_to_europe_2025() -> domain::entities::Group {
    TestGroup::new_valid(
        Uuid::parse_str("019b36d0b0ce72c7a1b46f44fcb55f22").unwrap(),
        "Trip to Europe 2025",
        Uuid::parse_str("019b14ef290a70d9a2452a4723d9d44a").unwrap(),
        Vec::<Uuid>::new(),
        DateTime::parse_from_rfc3339("2025-08-14T23:00:00Z")
            .unwrap()
            .to_utc(),
    )
}
