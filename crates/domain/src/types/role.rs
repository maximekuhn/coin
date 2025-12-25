use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Role {
    User,
    Moderator,
    Admin,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("unknown role")]
    Unknown,
}

impl FromStr for Role {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "user" => Ok(Self::User),
            "moderator" => Ok(Self::Moderator),
            "admin" => Ok(Self::Admin),
            _ => Err(Error::Unknown),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, Role};

    #[rstest::rstest]
    #[case("user", Role::User)]
    #[case("moderator", Role::Moderator)]
    #[case("admin", Role::Admin)]
    #[case("    user", Role::User)]
    #[case("user   ", Role::User)]
    #[case(" moderator     ", Role::Moderator)]
    #[case("admin ", Role::Admin)]
    fn valid_role(#[case] input: &str, #[case] expected_role: Role) {
        let role: Role = input.parse().unwrap();
        assert_eq!(expected_role, role);
    }

    #[rstest::rstest]
    #[case("")]
    #[case("usr")]
    #[case("ad")]
    #[case("mod")]
    #[case("u")]
    #[case("111111")]
    fn invalid_role(#[case] input: &str) {
        let err = input.parse::<Role>().unwrap_err();
        assert_eq!(Error::Unknown, err);
    }
}
