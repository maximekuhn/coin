use domain::types::user_id::UserId;
use sqlx::{
    FromRow,
    sqlite::SqliteRow,
    types::chrono::{DateTime, Utc},
};
use uuid::Uuid;

// -- related query: get_entry_by_user_id

#[derive(sqlx::FromRow)]
pub struct DbEntryWithSession {
    #[sqlx(rename = "entry_id")]
    pub id: Uuid,
    pub user_id: Uuid,
    pub hashed_password: Vec<u8>,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow)]
pub struct DbSessionForEntry {
    #[sqlx(rename = "session_id")]
    pub id: Vec<u8>,
    pub expires_at: DateTime<Utc>,
}

pub struct JoinDbEntryWithSession {
    pub entry: DbEntryWithSession,
    pub session: Option<DbSessionForEntry>,
}

impl<'r> FromRow<'r, SqliteRow> for JoinDbEntryWithSession {
    fn from_row(row: &SqliteRow) -> sqlx::Result<Self> {
        let entry = DbEntryWithSession::from_row(row)?;

        let session = match DbSessionForEntry::from_row(row) {
            Ok(s) => Some(s),
            Err(sqlx::Error::ColumnNotFound(_) | sqlx::Error::ColumnDecode { .. }) => None,
            Err(e) => return Err(e),
        };

        Ok(Self { entry, session })
    }
}

impl TryInto<auth_models::Entry> for JoinDbEntryWithSession {
    type Error = crate::Error;

    fn try_into(self) -> Result<auth_models::Entry, Self::Error> {
        let user_id =
            UserId::new(self.entry.user_id).map_err(|err| crate::Error::CorruptedData {
                msg: err.to_string(),
            })?;

        let session = match self.session {
            Some(session) => Some(try_into_session(session, self.entry.id)?),
            None => None,
        };

        Ok(auth_models::Entry {
            id: self.entry.id,
            user_id,
            hashed_password: self.entry.hashed_password,
            created_at: self.entry.created_at,
            sessions: match session {
                Some(s) => vec![s],
                None => vec![],
            },
        })
    }
}

fn try_into_session(
    db_session: DbSessionForEntry,
    entry_id: Uuid,
) -> Result<auth_models::Session, crate::Error> {
    let id = db_session
        .id
        .try_into()
        .map_err(|v: Vec<u8>| crate::Error::CorruptedData {
            msg: format!("auth_session.id: expected len to be 128, got {}", v.len()),
        })?;

    Ok(auth_models::Session {
        id,
        entry_id,
        expires_at: db_session.expires_at,
    })
}

#[derive(sqlx::FromRow)]
pub struct DbSession {
    id: Vec<u8>,
    auth_entry_id: Uuid,
    expires_at: DateTime<Utc>,
}

impl TryInto<auth_models::Session> for DbSession {
    type Error = crate::Error;

    fn try_into(self) -> Result<auth_models::Session, Self::Error> {
        let id = self
            .id
            .try_into()
            .map_err(|v: Vec<u8>| crate::Error::CorruptedData {
                msg: format!("auth_session.id: expected len to be 128, got {}", v.len()),
            })?;
        Ok(auth_models::Session {
            id,
            entry_id: self.auth_entry_id,
            expires_at: self.expires_at,
        })
    }
}
