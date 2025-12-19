use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Groupname {
    val: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("groupname cannot be empty")]
    Empty,

    #[error("groupname cannot exceed {} characters long", Groupname::MAX_LENGTH)]
    TooLong,
}

impl Groupname {
    const MAX_LENGTH: usize = 255;

    pub fn value(&self) -> String {
        self.val.clone()
    }
}

impl FromStr for Groupname {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err(Error::Empty);
        }
        if s.len() > Self::MAX_LENGTH {
            return Err(Error::TooLong);
        }
        Ok(Self { val: s.to_string() })
    }
}
