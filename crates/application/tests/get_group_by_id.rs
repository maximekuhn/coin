use application::queries::get_group_by_id::GetGroupByIdError;
use domain::types::group_id::GroupId;

use crate::infra::{ctx::TestContext, db::build_test_database};

mod infra;

const FILE: &str = file!();

#[tokio::test]
async fn not_found() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "not_found").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let bob_id = ctx.users().create_user("Bob").await?;

    // When
    let result = ctx
        .groups()
        .get_group_by_id(GroupId::new_random().value(), bob_id)
        .await?;

    // Then
    assert!(result.is_none());

    Ok(())
}

#[tokio::test]
async fn happy_path() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "happy_path").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let bob_id = ctx.users().create_user("Bob").await?;
    let group_id = ctx
        .groups()
        .create_empty_group("Bob's group", bob_id)
        .await?;

    // When
    let result = ctx.groups().get_group_by_id(group_id, bob_id).await?;

    // Then
    assert!(result.is_some());
    assert_eq!(group_id, result.unwrap().id.value());

    Ok(())
}

#[tokio::test]
async fn user_not_in_group() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "user_not_in_group").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let bob_id = ctx.users().create_user("Bob").await?;
    let alice_id = ctx.users().create_user("Alice").await?;

    let group_id = ctx
        .groups()
        .create_empty_group("Bob's group", bob_id)
        .await?;

    // When
    let result = ctx.groups().get_group_by_id(group_id, alice_id).await;

    // Then
    assert!(result.is_err());
    assert_eq!(
        GetGroupByIdError::CurrentUserNotInGroup.to_string(),
        result.unwrap_err().to_string()
    );

    Ok(())
}
