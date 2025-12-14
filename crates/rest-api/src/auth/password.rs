use std::str::FromStr;

pub struct Password {
    val: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("password cannot be empty")]
    Empty,

    #[error("password must be at least {} chars long", Password::MIN_LENGTH)]
    TooSmall,

    #[error("password must be at most {} chars long", Password::MAX_LENGTH)]
    TooLong,

    #[error("password is too weak")]
    TooWeak,
}

impl Password {
    const MIN_LENGTH: usize = 12;
    const MAX_LENGTH: usize = 128;

    pub fn value(&self) -> String {
        self.val.clone()
    }
}

impl FromStr for Password {
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

        let mut has_lowercase = false;
        let mut has_uppercase = false;
        let mut has_digit = false;
        let mut has_special = false;
        for c in s.chars() {
            if c.is_ascii_lowercase() {
                has_lowercase = true;
            } else if c.is_ascii_uppercase() {
                has_uppercase = true;
            } else if c.is_ascii_digit() {
                has_digit = true;
            } else {
                has_special = true;
            }
        }
        if has_lowercase && has_uppercase && has_digit && has_special {
            return Ok(Self { val: s.into() });
        }

        Err(Error::TooWeak)
    }
}
