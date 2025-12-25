use chrono::{DateTime, Utc};

pub fn jan_08_2025() -> DateTime<Utc> {
    DateTime::parse_from_rfc3339("2025-01-08T00:00:00Z")
        .unwrap()
        .to_utc()
}
