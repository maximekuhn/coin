use application::commands::create_empty_group::CreateEmptyGroupError;
use uuid::Uuid;

use crate::infra::{ctx::TestContext, db::build_test_database};

mod infra;

const FILE: &str = file!();

#[tokio::test]
async fn happy_path() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "happy_path").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let alice_id = ctx.users().create_user("Alice").await?;

    // When
    let _ = ctx
        .groups()
        .create_empty_group("Trip Summer 2026", alice_id)
        .await?;

    // Then
    ctx.groups()
        .assert_group_exists("Trip Summer 2026", alice_id)
        .await?;

    Ok(())
}

#[tokio::test]
async fn duplicate_name() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "duplicate_name").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let alice_id = ctx.users().create_user("Alice").await?;
    let _ = ctx
        .groups()
        .create_empty_group("Trip Summer 2026", alice_id)
        .await?;

    // When
    let err = ctx
        .groups()
        .create_empty_group("Trip Summer 2026", alice_id)
        .await
        .unwrap_err();

    // Then
    assert_eq!(
        CreateEmptyGroupError::NameNotAvailable.to_string(),
        err.to_string()
    );

    Ok(())
}

#[tokio::test]
async fn same_name_different_owners() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "same_name_different_owners").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let alice_id = ctx.users().create_user("Alice").await?;
    let bob_id = ctx.users().create_user("Bob").await?;

    // When
    let _ = ctx
        .groups()
        .create_empty_group("Trip Summer 2026", alice_id)
        .await?;
    let _ = ctx
        .groups()
        .create_empty_group("Trip Summer 2026", bob_id)
        .await?;

    // Then
    ctx.groups()
        .assert_group_exists("Trip Summer 2026", alice_id)
        .await?;
    ctx.groups()
        .assert_group_exists("Trip Summer 2026", bob_id)
        .await?;

    Ok(())
}

#[tokio::test]
async fn same_owner_different_names() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "same_owner_different_names").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let alice_id = ctx.users().create_user("Alice").await?;
    let _ = ctx
        .groups()
        .create_empty_group("Trip to Europe", alice_id)
        .await?;

    // When
    let _ = ctx
        .groups()
        .create_empty_group("Shared expenses - House", alice_id)
        .await?;

    // Then
    ctx.groups()
        .assert_group_exists("Trip to Europe", alice_id)
        .await?;
    ctx.groups()
        .assert_group_exists("Shared expenses - House", alice_id)
        .await?;

    Ok(())
}

#[tokio::test]
async fn same_owner_same_name_different_case() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "same_owner_same_name_different_case").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let alice_id = ctx.users().create_user("Alice").await?;
    let _ = ctx
        .groups()
        .create_empty_group("Trip to Europe", alice_id)
        .await?;

    // When
    let _ = ctx
        .groups()
        .create_empty_group("TRIP TO EUROPE", alice_id)
        .await?;

    // Then
    ctx.groups()
        .assert_group_exists("Trip to Europe", alice_id)
        .await?;
    ctx.groups()
        .assert_group_exists("TRIP TO EUROPE", alice_id)
        .await?;

    Ok(())
}

#[tokio::test]
async fn owner_not_found() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "owner_not_found").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    // When
    let err = ctx
        .groups()
        .create_empty_group("Trip Summer 2026", Uuid::now_v7())
        .await
        .unwrap_err();

    // Then
    assert_eq!(
        CreateEmptyGroupError::OwnerNotFound.to_string(),
        err.to_string()
    );

    Ok(())
}
