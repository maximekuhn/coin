use domain::{entities::User, types::user_id::UserId};
use email_address::EmailAddress;
use uuid::Uuid;

use crate::models::user::{DbUser, db_role::DbRole};

pub async fn email_exists(
    tx: &mut crate::Transaction<'_>,
    email: &EmailAddress,
) -> Result<bool, crate::Error> {
    Ok(sqlx::query_as::<_, (Uuid,)>(
        r#"
        SELECT id
        FROM user
        WHERE email = ?
        "#,
    )
    .bind(email.email())
    .fetch_optional(tx.as_mut())
    .await?
    .is_some())
}

pub async fn create(
    tx: &mut crate::Transaction<'_>,
    user: &domain::entities::User,
) -> Result<(), crate::Error> {
    sqlx::query(
        r#"
    INSERT INTO user (id, name, email, role, created_at)
    VALUES (?, ?, ? ,?, ?)
    "#,
    )
    .bind(user.id.value())
    .bind(user.name.value())
    .bind(user.email.email())
    .bind(DbRole::from(&user.role).0)
    .bind(user.created_at)
    .execute(tx.as_mut())
    .await?;
    Ok(())
}

pub async fn get_by_email(
    tx: &mut crate::Transaction<'_>,
    email: &EmailAddress,
) -> Result<Option<User>, crate::Error> {
    let row: Option<DbUser> = sqlx::query_as(
        r#"
    SELECT *
    FROM user
    WHERE email = ?
    "#,
    )
    .bind(email.email())
    .fetch_optional(tx.as_mut())
    .await?;

    let Some(row) = row else {
        return Ok(None);
    };
    Ok(Some(row.try_into()?))
}

pub async fn get_by_id(
    tx: &mut crate::Transaction<'_>,
    id: &UserId,
) -> Result<Option<User>, crate::Error> {
    let row: Option<DbUser> = sqlx::query_as(
        r#"
    SELECT *
    FROM user
    WHERE id = ?
    "#,
    )
    .bind(id.value())
    .fetch_optional(tx.as_mut())
    .await?;

    let Some(row) = row else {
        return Ok(None);
    };
    Ok(Some(row.try_into()?))
}

pub async fn exists_by_id(
    tx: &mut crate::Transaction<'_>,
    id: &UserId,
) -> Result<bool, crate::Error> {
    Ok(sqlx::query_as::<_, (Uuid,)>(
        r#"
        SELECT id
        FROM user
        WHERE id = ?
        "#,
    )
    .bind(id.value())
    .fetch_optional(tx.as_mut())
    .await?
    .is_some())
}
