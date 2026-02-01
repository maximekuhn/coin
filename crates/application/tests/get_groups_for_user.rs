use chrono::Utc;

use crate::infra::{ctx::TestContext, db::build_test_database};

mod infra;

const FILE: &str = file!();

#[tokio::test]
async fn happy_path_empty() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "happy_path_empty").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let bob_id = ctx.users().create_user("Bob").await?;

    // When
    let output = ctx.groups().get_all_for_user(bob_id).await?;

    // Then
    assert!(output.groups.is_empty());
    assert_eq!(0, output.total_items);

    Ok(())
}

#[tokio::test]
async fn happy_path_single_group() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "happy_path_single_group").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let bob_id = ctx.users().create_user("Bob").await?;
    ctx.groups()
        .create_empty_group("Bob's group", bob_id)
        .await?;

    // When
    let output = ctx.groups().get_all_for_user(bob_id).await?;

    // Then
    assert_eq!(1, output.groups.len());
    assert_eq!(1, output.total_items);

    let group = output.groups.first().unwrap();
    assert_eq!("Bob's group", group.name.value());
    assert_eq!(bob_id, group.owner.id.value());
    assert_eq!("Bob", group.owner.name.value());
    assert!(group.last_expense.is_none());
    assert!(group.current_user_balance.is_none());

    Ok(())
}

#[tokio::test]
async fn happy_path_single_group_with_expense() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "happy_path_single_group_with_expense").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let bob_id = ctx.users().create_user("Bob").await?;
    let group_id = ctx
        .groups()
        .create_empty_group("Bob's group", bob_id)
        .await?;
    let expense_id = ctx
        .expense_entries()
        .create_expense_for_all_group_members(group_id, bob_id, 27, bob_id, Utc::now())
        .await?;

    // When
    let output = ctx.groups().get_all_for_user(bob_id).await?;

    // Then
    assert_eq!(1, output.groups.len());
    assert_eq!(1, output.total_items);

    let group = output.groups.first().unwrap();
    assert_eq!(expense_id, group.last_expense.as_ref().unwrap().id.value());
    assert!(group.current_user_balance.is_none());

    Ok(())
}

#[tokio::test]
async fn happy_path_single_group_with_multiple_users_and_expenses() -> anyhow::Result<()> {
    let db_pool = build_test_database(
        FILE,
        "happy_path_single_group_with_multiple_users_and_expenses",
    )
    .await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let bob_id = ctx.users().create_user("Bob").await?;
    let alice_id = ctx.users().create_user("Alice").await?;
    let charlie_id = ctx.users().create_user("Charlie").await?;

    let group_id = ctx
        .groups()
        .create_empty_group("Bob's group", bob_id)
        .await?;
    ctx.groups().add_member(group_id, bob_id, alice_id).await?;
    ctx.groups()
        .add_member(group_id, bob_id, charlie_id)
        .await?;

    ctx.expense_entries()
        .create_expense_for_all_group_members(group_id, bob_id, 50, bob_id, Utc::now())
        .await?;

    let expense_id_2 = ctx
        .expense_entries()
        .create_expense(
            group_id,
            charlie_id,
            20,
            vec![alice_id],
            charlie_id,
            Utc::now(),
        )
        .await?;

    // When
    let output = ctx.groups().get_all_for_user(bob_id).await?;

    // Then
    assert_eq!(1, output.groups.len());
    assert_eq!(1, output.total_items);

    let group = output.groups.first().unwrap();
    assert_eq!(
        expense_id_2,
        group.last_expense.as_ref().unwrap().id.value()
    );
    assert_eq!(16, group.current_user_balance.as_ref().unwrap().euros());

    Ok(())
}

#[tokio::test]
async fn happy_path_other_user_as_group_owner() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "happy_path_other_user_as_group_owner").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let bob_id = ctx.users().create_user("Bob").await?;
    let alice_id = ctx.users().create_user("Alice").await?;
    let alice_group_id = ctx
        .groups()
        .create_empty_group("Alice's group", alice_id)
        .await?;
    ctx.groups()
        .add_member(alice_group_id, alice_id, bob_id)
        .await?;

    // When
    let output = ctx.groups().get_all_for_user(bob_id).await?;

    // Then
    assert_eq!(1, output.groups.len());
    assert_eq!(1, output.total_items);

    assert_eq!(alice_group_id, output.groups.first().unwrap().id.value());

    Ok(())
}

#[tokio::test]
async fn happy_path_multiple_groups() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "happy_path_multiple_groups").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let bob_id = ctx.users().create_user("Bob").await?;
    let alice_id = ctx.users().create_user("Alice").await?;

    let bob_group_id = ctx
        .groups()
        .create_empty_group("Bob's group", bob_id)
        .await?;
    let alice_group_id = ctx
        .groups()
        .create_empty_group("Alice's group", alice_id)
        .await?;
    ctx.groups()
        .add_member(alice_group_id, alice_id, bob_id)
        .await?;

    // When
    let output = ctx.groups().get_all_for_user(bob_id).await?;

    // Then
    assert_eq!(2, output.groups.len());
    assert_eq!(2, output.total_items);

    let first_group = output.groups.first().unwrap();
    let second_group = output.groups.last().unwrap();
    assert_eq!(alice_group_id, first_group.id.value());
    assert_eq!(alice_id, first_group.owner.id.value());
    assert_eq!(bob_group_id, second_group.id.value());
    assert_eq!(bob_id, second_group.owner.id.value());

    Ok(())
}

#[tokio::test]
async fn happy_path_multiple_groups_pagination_works() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "happy_path_multiple_groups_pagination_works").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let bob_id = ctx.users().create_user("Bob").await?;
    let alice_id = ctx.users().create_user("Alice").await?;

    let bob_group_id = ctx
        .groups()
        .create_empty_group("Bob's group", bob_id)
        .await?;
    let alice_group_id = ctx
        .groups()
        .create_empty_group("Alice's group", alice_id)
        .await?;
    ctx.groups()
        .add_member(alice_group_id, alice_id, bob_id)
        .await?;

    // When
    let output = ctx
        .groups()
        .get_all_for_user_with_pagination(bob_id, 2, 1)
        .await?;

    // Then
    assert_eq!(1, output.groups.len());
    assert_eq!(2, output.total_items);

    assert_eq!(bob_group_id, output.groups.first().unwrap().id.value());

    Ok(())
}

#[tokio::test]
async fn happy_path_user_with_no_group() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "happy_path_user_with_no_group").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let bob_id = ctx.users().create_user("Bob").await?;
    let alice_id = ctx.users().create_user("Alice").await?;
    let charlie_id = ctx.users().create_user("Charlie").await?;

    ctx.groups()
        .create_empty_group("Bob's group", bob_id)
        .await?;
    let alice_group_id = ctx
        .groups()
        .create_empty_group("Alice's group", alice_id)
        .await?;
    ctx.groups()
        .add_member(alice_group_id, alice_id, bob_id)
        .await?;

    // When
    let output = ctx.groups().get_all_for_user(charlie_id).await?;

    // Then
    assert!(output.groups.is_empty());
    assert_eq!(0, output.total_items);

    Ok(())
}
