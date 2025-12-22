use application::commands::add_group_member::AddGroupMemberError;
use uuid::Uuid;

use crate::infra::{ctx::TestContext, db::build_test_database};

mod infra;

const FILE: &str = file!();

#[tokio::test]
async fn happy_path() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "happy_path").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let bob_id = ctx.users().create_user("Bob").await?;
    let alice_id = ctx.users().create_user("Alice").await?;
    let group_id = ctx
        .groups()
        .create_empty_group("Trip Summer 2026", alice_id)
        .await?;

    // When
    ctx.groups().add_member(group_id, alice_id, bob_id).await?;

    // Then
    ctx.groups()
        .assert_group_contains_members(group_id, vec![bob_id])
        .await?;

    Ok(())
}

#[tokio::test]
async fn owner_cannot_self_add() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "owner_cannot_self_add").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let alice_id = ctx.users().create_user("Alice").await?;
    let group_id = ctx
        .groups()
        .create_empty_group("Trip Summer 2026", alice_id)
        .await?;

    // When
    let err = ctx
        .groups()
        .add_member(group_id, alice_id, alice_id)
        .await
        .unwrap_err();

    // Then
    assert_eq!(
        AddGroupMemberError::AlreadyMember.to_string(),
        err.to_string()
    );

    Ok(())
}

#[tokio::test]
async fn user_to_add_not_found() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "user_to_add_not_found").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let alice_id = ctx.users().create_user("Alice").await?;
    let group_id = ctx
        .groups()
        .create_empty_group("Trip Summer 2026", alice_id)
        .await?;

    // When
    let err = ctx
        .groups()
        .add_member(group_id, alice_id, Uuid::now_v7())
        .await
        .unwrap_err();

    // Then
    assert_eq!(
        AddGroupMemberError::UserNotFound.to_string(),
        err.to_string()
    );

    Ok(())
}

#[tokio::test]
async fn group_not_found() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "group_not_found").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let alice_id = ctx.users().create_user("Alice").await?;
    let bob_id = ctx.users().create_user("Bob").await?;

    // When
    let err = ctx
        .groups()
        .add_member(Uuid::now_v7(), alice_id, bob_id)
        .await
        .unwrap_err();

    // Then
    assert_eq!(
        AddGroupMemberError::GroupNotFound.to_string(),
        err.to_string()
    );

    Ok(())
}

#[tokio::test]
async fn user_already_member() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "user_already_member").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let alice_id = ctx.users().create_user("Alice").await?;
    let bob_id = ctx.users().create_user("Bob").await?;
    let group_id = ctx
        .groups()
        .create_empty_group("Trip Summer 2026", alice_id)
        .await?;
    let _ = ctx.groups().add_member(group_id, alice_id, bob_id).await?;

    // When
    let err = ctx
        .groups()
        .add_member(group_id, alice_id, bob_id)
        .await
        .unwrap_err();

    // Then
    assert_eq!(
        AddGroupMemberError::AlreadyMember.to_string(),
        err.to_string()
    );

    Ok(())
}

#[tokio::test]
async fn owner_not_found() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "owner_not_found").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let alice_id = ctx.users().create_user("Alice").await?;
    let bob_id = ctx.users().create_user("Bob").await?;
    let group_id = ctx
        .groups()
        .create_empty_group("Trip Summer 2026", alice_id)
        .await?;

    // When
    let err = ctx
        .groups()
        .add_member(group_id, Uuid::now_v7(), bob_id)
        .await
        .unwrap_err();

    // Then
    assert_eq!(AddGroupMemberError::NotOwner.to_string(), err.to_string());

    Ok(())
}
