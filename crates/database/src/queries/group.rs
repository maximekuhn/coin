use domain::{
    entities::Group,
    types::{group_id::GroupId, groupname::Groupname, user_id::UserId},
};
use sqlx::QueryBuilder;
use uuid::Uuid;

use crate::models::group::{DbGroup, DbGroupMember, DbGroupWithMembers};

pub async fn exists_by_name_for_owner(
    tx: &mut crate::Transaction<'_>,
    groupname: &Groupname,
    owner_id: &UserId,
) -> Result<bool, crate::Error> {
    Ok(sqlx::query_as::<_, (Uuid,)>(
        r#"
        SELECT id
        FROM coin_group
        WHERE name = ?
        AND owner_id = ?
        "#,
    )
    .bind(groupname.value())
    .bind(owner_id.value())
    .fetch_optional(tx.as_mut())
    .await?
    .is_some())
}

pub async fn create(
    tx: &mut crate::Transaction<'_>,
    group: &domain::entities::Group,
) -> Result<(), crate::Error> {
    let group_id = group.id.value();
    sqlx::query(
        r#"
    INSERT INTO coin_group (id, name, owner_id, created_at) VALUES
    (?, ?, ?, ?)
    "#,
    )
    .bind(group_id)
    .bind(group.name.value())
    .bind(group.owner_id.value())
    .bind(group.created_at)
    .execute(tx.as_mut())
    .await?;

    if group.members.is_empty() {
        return Ok(());
    }

    let mut qb = QueryBuilder::new(
        r#"
    INSERT INTO coin_group_member (coin_group_id, member_id)
    "#,
    );
    qb.push_values(&group.members, |mut b, member_id| {
        b.push_bind(group_id).push_bind(member_id.value());
    });
    qb.build().execute(tx.as_mut()).await?;

    Ok(())
}

pub async fn get_by_id(
    tx: &mut crate::Transaction<'_>,
    id: &GroupId,
) -> Result<Option<Group>, crate::Error> {
    let group: Option<DbGroup> = sqlx::query_as(
        r#"
    SELECT
        id,
        name,
        owner_id,
        created_at
    FROM coin_group cg
    WHERE cg.id = ?
    "#,
    )
    .bind(id.value())
    .fetch_optional(tx.as_mut())
    .await?;

    let Some(group) = group else {
        return Ok(None);
    };

    let members: Vec<DbGroupMember> = sqlx::query_as(
        r#"
    SELECT coin_group_id, member_id
    FROM coin_group_member
    WHERE coin_group_id = ?
    "#,
    )
    .bind(id.value())
    .fetch_all(tx.as_mut())
    .await?;

    Ok(Some(DbGroupWithMembers { group, members }.try_into()?))
}

pub async fn add_member(
    tx: &mut crate::Transaction<'_>,
    id: &GroupId,
    user_id: &UserId,
) -> Result<(), crate::Error> {
    sqlx::query(
        r#"
    INSERT INTO coin_group_member (coin_group_id, member_id)
    VALUES (?, ?)
    "#,
    )
    .bind(id.value())
    .bind(user_id.value())
    .execute(tx.as_mut())
    .await?;
    Ok(())
}
