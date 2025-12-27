use domain::{
    entities::Group,
    types::{group_id::GroupId, groupname::Groupname, user_id::UserId},
};
use sqlx::QueryBuilder;
use uuid::Uuid;

use crate::{
    DbPagination,
    models::group::{
        DbGroup, DbGroupMember, DbGroupWithMember, DbGroupWithMembers, flatten_group_with_member,
    },
};

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

/// Returns all group that contains the provided `user_id` as owner or member.
///
/// # Arguments
/// - `tx`
/// - `user_id`
/// - `page` pagination to apply to groups
///
/// # Return
/// - a list of groups, sorted by creation date (DESC) and id
pub async fn get_all_for_user(
    tx: &mut crate::Transaction<'_>,
    user_id: &UserId,
    pagination: DbPagination,
) -> Result<Vec<Group>, crate::Error> {
    let group_ids: Vec<(Uuid,)> = sqlx::query_as(
        r#"
    SELECT DISTINCT cg.id
    FROM coin_group cg
    LEFT JOIN coin_group_member cgm ON cgm.coin_group_id = cg.id
    WHERE cg.owner_id = ?
    OR cgm.member_id = ?
    ORDER BY cg.created_at DESC, cg.id
    LIMIT ? OFFSET ?
    "#,
    )
    .bind(user_id.value())
    .bind(user_id.value())
    .bind(pagination.limit as i64)
    .bind(pagination.offset as i64)
    .fetch_all(tx.as_mut())
    .await?;

    if group_ids.is_empty() {
        return Ok(vec![]);
    }

    let group_ids: Vec<Uuid> = group_ids.into_iter().map(|g| g.0).collect();
    let placeholders = std::iter::repeat_n("?", group_ids.len())
        .collect::<Vec<_>>()
        .join(", ");

    let sql = format!(
        r#"
    SELECT
        cg.id,
        cg.name,
        cg.owner_id,
        cg.created_at,
        cgm.member_id
    FROM coin_group cg
    LEFT JOIN coin_group_member cgm ON cgm.coin_group_id = cg.id
    WHERE cg.id IN ({})
    "#,
        placeholders
    );

    let mut query = sqlx::query_as::<_, DbGroupWithMember>(&sql);
    for id in &group_ids {
        query = query.bind(id);
    }
    let rows = query.fetch_all(tx.as_mut()).await?;
    flatten_group_with_member(rows)
}

pub async fn count_all_for_user(
    tx: &mut crate::Transaction<'_>,
    user_id: &UserId,
) -> Result<u64, crate::Error> {
    let count: i64 = sqlx::query_scalar(
        r#"
    SELECT COUNT(DISTINCT g.id)
    FROM coin_group g
    LEFT JOIN coin_group_member gm ON gm.coin_group_id = g.id
    WHERE g.owner_id = ?
    OR gm.member_id = ?
    "#,
    )
    .bind(user_id.value())
    .bind(user_id.value())
    .fetch_one(tx.as_mut())
    .await?;
    Ok(count as u64)
}
