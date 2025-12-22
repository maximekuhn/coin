use application::commands::create_user::CreateUserError;
use domain::types::role::Role;

use crate::infra::{ctx::TestContext, db::build_test_database};

mod infra;

const FILE: &str = file!();

#[tokio::test]
async fn happy_path() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "happy_path").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    // When
    let user_id = ctx
        .users()
        .create_user_with_name_and_email("Bob", "bob@gmail.com")
        .await?;

    // Then
    ctx.users()
        .assert_user_exists(user_id, "Bob", "bob@gmail.com", Role::User)
        .await?;

    Ok(())
}

#[tokio::test]
async fn email_already_taken() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "email_already_taken").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let _ = ctx
        .users()
        .create_user_with_name_and_email("Bob", "bob.thegoat@gmail.com")
        .await?;

    // When
    let err = ctx
        .users()
        .create_user_with_name_and_email("Josh", "bob.thegoat@gmail.com")
        .await
        .unwrap_err();

    // Then
    assert_eq!(
        CreateUserError::EmailAlreadyTaken.to_string(),
        err.to_string()
    );

    Ok(())
}

#[tokio::test]
async fn same_username_different_email() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "same_username_different_email").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let existing_user_id = ctx
        .users()
        .create_user_with_name_and_email("Bob", "bob.thegoat@gmail.com")
        .await?;

    // When
    let new_user_id = ctx
        .users()
        .create_user_with_name_and_email("Bob", "bob2@gmail.com")
        .await?;

    // Then
    ctx.users()
        .assert_user_exists(existing_user_id, "Bob", "bob.thegoat@gmail.com", Role::User)
        .await?;
    ctx.users()
        .assert_user_exists(new_user_id, "Bob", "bob2@gmail.com", Role::User)
        .await?;

    Ok(())
}
