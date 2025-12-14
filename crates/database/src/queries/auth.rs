use domain::types::user_id::UserId;
use uuid::Uuid;

use crate::models::auth::{DbSession, JoinDbEntryWithSession};

pub async fn entry_exists_for_user_id(
    tx: &mut crate::Transaction<'_>,
    user_id: &UserId,
) -> Result<bool, crate::Error> {
    Ok(sqlx::query_as::<_, (Uuid,)>(
        r#"
        SELECT id
        FROM auth_entry
        WHERE id = ?
        "#,
    )
    .bind(user_id.value())
    .fetch_optional(tx.as_mut())
    .await?
    .is_some())
}

pub async fn create_entry(
    tx: &mut crate::Transaction<'_>,
    entry: &auth_models::Entry,
) -> Result<(), crate::Error> {
    sqlx::query(
        r#"
    INSERT INTO auth_entry (id, user_id, hashed_password, created_at)
    VALUES (?, ?, ?, ?)
    "#,
    )
    .bind(entry.id)
    .bind(entry.user_id.value())
    .bind(entry.hashed_password.clone())
    .bind(entry.created_at)
    .execute(tx.as_mut())
    .await?;
    Ok(())
}

pub async fn get_entry_by_user_id(
    tx: &mut crate::Transaction<'_>,
    user_id: &UserId,
) -> Result<Option<auth_models::Entry>, crate::Error> {
    let rows: Vec<JoinDbEntryWithSession> = sqlx::query_as(
        r#"
    SELECT
        e.id AS entry_id,
        e.user_id,
        e.hashed_password,
        e.created_at,
        s.id AS session_id,
        s.expires_at
    FROM auth_entry e
    LEFT JOIN auth_session s ON e.id = s.auth_entry_id
    WHERE e.user_id = ?
    "#,
    )
    .bind(user_id.value())
    .fetch_all(tx.as_mut())
    .await?;

    if rows.is_empty() {
        return Ok(None);
    }

    let entries: Vec<auth_models::Entry> = rows
        .into_iter()
        .map(TryInto::try_into)
        .collect::<Result<_, _>>()?;
    let mut iter = entries.into_iter();
    let mut entry = iter.next().expect("entries has at least 1 element");
    for e in iter {
        entry.sessions.extend(e.sessions);
    }
    Ok(Some(entry))
}

pub async fn get_entry_by_id(
    tx: &mut crate::Transaction<'_>,
    entry_id: &Uuid,
) -> Result<Option<auth_models::Entry>, crate::Error> {
    let rows: Vec<JoinDbEntryWithSession> = sqlx::query_as(
        r#"
    SELECT
        e.id AS entry_id,
        e.user_id,
        e.hashed_password,
        e.created_at,
        s.id AS session_id,
        s.expires_at
    FROM auth_entry e
    LEFT JOIN auth_session s ON e.id = s.auth_entry_id
    WHERE e.id = ?
    "#,
    )
    .bind(entry_id)
    .fetch_all(tx.as_mut())
    .await?;

    if rows.is_empty() {
        return Ok(None);
    }

    let entries: Vec<auth_models::Entry> = rows
        .into_iter()
        .map(TryInto::try_into)
        .collect::<Result<_, _>>()?;
    let mut iter = entries.into_iter();
    let mut entry = iter.next().expect("entries has at least 1 element");
    for e in iter {
        entry.sessions.extend(e.sessions);
    }
    Ok(Some(entry))
}

pub async fn delete_session(
    tx: &mut crate::Transaction<'_>,
    entry_id: &Uuid,
    session_id: &[u8; 128],
) -> Result<(), crate::Error> {
    sqlx::query(
        r#"
    DELETE FROM auth_session
    WHERE id = ?
    AND auth_entry_id = ?
    "#,
    )
    .bind(session_id.to_vec())
    .bind(entry_id)
    .execute(tx.as_mut())
    .await?;
    Ok(())
}

pub async fn create_session(
    tx: &mut crate::Transaction<'_>,
    session: &auth_models::Session,
) -> Result<(), crate::Error> {
    sqlx::query(
        r#"
    INSERT INTO auth_session (id, auth_entry_id, expires_at)
    VALUES (?, ?, ?)
    "#,
    )
    .bind(session.id.to_vec())
    .bind(session.entry_id)
    .bind(session.expires_at)
    .execute(tx.as_mut())
    .await?;
    Ok(())
}

pub async fn get_session_by_id(
    tx: &mut crate::Transaction<'_>,
    session_id: [u8; 128],
) -> Result<Option<auth_models::Session>, crate::Error> {
    let row: Option<DbSession> = sqlx::query_as(
        r#"
    SELECT id, auth_entry_id, expires_at
    FROM auth_session
    WHERE id = ?
    "#,
    )
    .bind(session_id.to_vec())
    .fetch_optional(tx.as_mut())
    .await?;
    let Some(row) = row else {
        return Ok(None);
    };
    Ok(Some(row.try_into()?))
}
