use std::str::FromStr;

use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct UserId {
    val: Uuid,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("id must be a valid UUID")]
    Malformed,

    #[error("id cannot be zero UUID")]
    ZerosOnly,

    #[error("id cannot be ones only UUID")]
    OnesOnly,
}

impl UserId {
    pub fn new(id: Uuid) -> Result<Self, Error> {
        if id.is_nil() {
            return Err(Error::ZerosOnly);
        }
        if id.is_max() {
            return Err(Error::OnesOnly);
        }
        Ok(Self { val: id })
    }

    pub fn new_random() -> Self {
        Self::new(Uuid::now_v7()).expect("valid UUID v7")
    }

    pub fn value(&self) -> Uuid {
        self.val
    }
}

impl FromStr for UserId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id: Uuid = s.trim().parse().map_err(|_| Error::Malformed)?;
        Self::new(id)
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, UserId};

    #[rstest::rstest]
    #[case("019b14ca-c11a-7882-ac00-0e88e8ba5e84")]
    #[case("019b14cac11a7882ac000e88e8ba5e84")]
    #[case("      019b14cac11a7882ac000e88e8ba5e84")]
    #[case("019b14ca-c11a-7882-ac00-0e88e8ba5e84     ")]
    fn valid_user_id(#[case] input: &str) {
        let user_id: UserId = input.parse().unwrap();
        assert_eq!(
            input.trim().replace('-', ""),
            user_id.value().to_string().replace('-', "")
        );
    }

    #[rstest::rstest]
    #[case("", Error::Malformed)]
    #[case("00000000-0000-0000-0000-000000000000", Error::ZerosOnly)]
    #[case("00000000000000000000000000000000", Error::ZerosOnly)]
    #[case("ffffffff-ffff-ffff-ffff-ffffffffffff", Error::OnesOnly)]
    #[case("ffffffffffffffffffffffffffffffff", Error::OnesOnly)]
    fn invalid_user_id(#[case] input: &str, #[case] expected_err: Error) {
        let err = input.parse::<UserId>().unwrap_err();
        assert_eq!(expected_err, err);
    }

    #[test]
    fn new_random_works() {
        let id = UserId::new_random();
        assert!(!id.value().is_nil());
        assert!(!id.value().is_max());
    }
}
