#[macro_export]
macro_rules! id_type {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name {
            val: ::uuid::Uuid,
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

        impl $name {
            pub fn new(id: ::uuid::Uuid) -> Result<Self, Error> {
                if id.is_nil() {
                    return Err(Error::ZerosOnly);
                }
                if id.is_max() {
                    return Err(Error::OnesOnly);
                }
                Ok(Self { val: id })
            }

            pub fn new_random() -> Self {
                Self::new(::uuid::Uuid::now_v7()).expect("valid UUID v7")
            }

            pub fn value(&self) -> ::uuid::Uuid {
                self.val
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let id: ::uuid::Uuid = s.trim().parse().map_err(|_| Error::Malformed)?;
                Self::new(id)
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", self.val)
            }
        }
    };
}
