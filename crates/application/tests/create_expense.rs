use std::collections::HashSet;

use application::commands::create_expense::CreateExpenseError;
use domain::types::user_id::UserId;
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
        .create_empty_group("Bali Trip 2026", bob_id)
        .await?;
    ctx.groups().add_member(group_id, bob_id, alice_id).await?;

    // When
    let expense_entry_id = ctx
        .expense_entries()
        .create_expense_for_all_group_members(group_id, bob_id, 128)
        .await?;

    // Then
    let expense_entry = ctx
        .expense_entries()
        .assert_entry_exists(expense_entry_id)
        .await?;

    assert_eq!(bob_id, expense_entry.paid_by.value());
    assert_eq!(
        HashSet::from_iter(vec![UserId::new(alice_id)?]),
        expense_entry.participants
    );
    assert_eq!(group_id, expense_entry.group_id.value());
    assert_eq!(12_800, expense_entry.total.cents());

    Ok(())
}

#[tokio::test]
async fn happy_path_list_participants() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "happy_path_list_participants").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let alice_id = ctx.users().create_user("Alice").await?;
    let bob_id = ctx.users().create_user("Bob").await?;
    let _ = ctx.users().create_user("Charlie").await?;
    let group_id = ctx
        .groups()
        .create_empty_group("Bali Trip 2026", bob_id)
        .await?;
    ctx.groups().add_member(group_id, bob_id, alice_id).await?;

    // When
    let expense_entry_id = ctx
        .expense_entries()
        .create_expense(group_id, alice_id, 56, vec![bob_id])
        .await?;

    // Then
    let expense_entry = ctx
        .expense_entries()
        .assert_entry_exists(expense_entry_id)
        .await?;
    assert_eq!(alice_id, expense_entry.paid_by.value());

    Ok(())
}

#[tokio::test]
async fn negative_total() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "negative_total").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let bob_id = ctx.users().create_user("Bob").await?;
    let alice_id = ctx.users().create_user("Alice").await?;
    let group_id = ctx
        .groups()
        .create_empty_group("Bali Trip 2026", bob_id)
        .await?;
    ctx.groups().add_member(group_id, bob_id, alice_id).await?;

    // When
    let err = ctx
        .expense_entries()
        .create_expense_for_all_group_members(group_id, bob_id, -10_000)
        .await
        .unwrap_err();

    // Then
    assert_eq!(
        CreateExpenseError::NegativeTotal.to_string(),
        err.to_string()
    );

    Ok(())
}

#[tokio::test]
async fn group_not_found() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "group_not_found").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let bob_id = ctx.users().create_user("Bob").await?;

    // When
    let err = ctx
        .expense_entries()
        .create_expense_for_all_group_members(Uuid::now_v7(), bob_id, 50)
        .await
        .unwrap_err();

    // Then
    assert_eq!(
        CreateExpenseError::GroupNotFound.to_string(),
        err.to_string()
    );

    Ok(())
}

#[tokio::test]
async fn payer_not_in_group() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "payer_not_in_group").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let bob_id = ctx.users().create_user("Bob").await?;
    let alice_id = ctx.users().create_user("Alice").await?;
    let group_id = ctx
        .groups()
        .create_empty_group("Alice's expenses", alice_id)
        .await?;

    // When
    let err = ctx
        .expense_entries()
        .create_expense_for_all_group_members(group_id, bob_id, 23)
        .await
        .unwrap_err();

    // Then
    assert_eq!(
        CreateExpenseError::PayerIsNotGroupMember.to_string(),
        err.to_string()
    );

    Ok(())
}

#[tokio::test]
async fn participant_not_found_in_group() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "participant_not_found_in_group").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let alice_id = ctx.users().create_user("Alice").await?;
    let bob_id = ctx.users().create_user("Bob").await?;
    let charlie_id = ctx.users().create_user("Charlie").await?;
    let group_id = ctx
        .groups()
        .create_empty_group("Bob and Alice expenses", alice_id)
        .await?;

    // When
    let err = ctx
        .expense_entries()
        .create_expense(
            group_id,
            alice_id,
            5_031,
            vec![alice_id, bob_id, charlie_id],
        )
        .await
        .unwrap_err();

    // Then
    assert_eq!(
        CreateExpenseError::ParticipantNotFound.to_string(),
        err.to_string()
    );

    Ok(())
}
