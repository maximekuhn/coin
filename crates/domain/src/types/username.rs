use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct Username {
    val: String,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("username cannot be empty")]
    Empty,

    #[error(
        "username length must be at least {} characters long",
        Username::MIN_LENGTH
    )]
    TooSmall,

    #[error("username length cannot exceed {} characters", Username::MAX_LENGTH)]
    TooLong,

    #[error("username must start with letter")]
    MustStartWithLetter,

    #[error("username can only contain letters, numbers, dashes and underscores")]
    InvalidChars,
}

impl Username {
    const MIN_LENGTH: usize = 3;
    const MAX_LENGTH: usize = 24;

    pub fn value(&self) -> String {
        self.val.clone()
    }
}

impl FromStr for Username {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s.is_empty() {
            return Err(Error::Empty);
        }

        if s.len() < Self::MIN_LENGTH {
            return Err(Error::TooSmall);
        }
        if s.len() > Self::MAX_LENGTH {
            return Err(Error::TooLong);
        }

        let first_char = s.chars().next().expect("s.len() > Self::MIN_LENGTH");
        if !first_char.is_alphabetic() {
            return Err(Error::MustStartWithLetter);
        }

        let not_allowed_chars = s
            .chars()
            .filter(|c| !is_allowed_char(c))
            .collect::<Vec<_>>();
        if !not_allowed_chars.is_empty() {
            return Err(Error::InvalidChars);
        }

        Ok(Self { val: s.into() })
    }
}

fn is_allowed_char(c: &char) -> bool {
    c.is_alphanumeric() || *c == '-' || *c == '_'
}

impl std::fmt::Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, Username};

    #[rstest::rstest]
    #[case("abc")]
    #[case("A_user-123")]
    #[case("UserName_")]
    #[case("john_doe")]
    #[case("alice123")]
    #[case("Bob-Builder")]
    #[case("xYz_09")]
    #[case("   trimmed   ")]
    #[case("usér")]
    #[case("ąbc")]
    fn valid_username(#[case] input: &str) {
        let username: Username = input.parse().unwrap();
        assert_eq!(input.trim(), username.value());
    }

    #[rstest::rstest]
    #[case("", Error::Empty)]
    #[case("a", Error::TooSmall)]
    #[case("ab", Error::TooSmall)]
    #[case("aaaaaaaaaaaaaaaaaaaaaaaaa", Error::TooLong)]
    #[case("abcdefghijklmnopqrstuvwxyz", Error::TooLong)]
    #[case("1abc", Error::MustStartWithLetter)]
    #[case("_abc", Error::MustStartWithLetter)]
    #[case("-user", Error::MustStartWithLetter)]
    #[case("john!", Error::InvalidChars)]
    #[case("john doe", Error::InvalidChars)]
    #[case("john$", Error::InvalidChars)]
    #[case("jo@n", Error::InvalidChars)]
    #[case("   ", Error::Empty)]
    fn invalid_username(#[case] input: &str, #[case] expected_err: Error) {
        let err = input.parse::<Username>().unwrap_err();
        assert_eq!(expected_err, err);
    }
}
