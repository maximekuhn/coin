use application::{
    models::{group::GroupSummary, user::UserSummary},
    queries::get_latest_expenses_for_user::ExpenseSummary,
};
use chrono::Utc;
use domain::types::{expense_id::ExpenseId, group_id::GroupId, money::Money, user_id::UserId};

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
    let output = ctx.expense_entries().get_latest_expenses(bob_id).await?;

    // Then
    assert!(output.expenses.is_empty());
    assert_eq!(0, output.total_items);

    Ok(())
}

#[tokio::test]
async fn happy_path_single_expense() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "happy_path_single_expense").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let bob_id = ctx.users().create_user("Bob").await?;
    let group_id = ctx
        .groups()
        .create_empty_group("Bob's group", bob_id)
        .await?;
    let expense_id = ctx
        .expense_entries()
        .create_expense_for_all_group_members(group_id, bob_id, 54, bob_id, Utc::now())
        .await?;

    // When
    let output = ctx.expense_entries().get_latest_expenses(bob_id).await?;

    // Then
    assert_eq!(1, output.expenses.len());
    assert_eq!(1, output.total_items);

    assert_eq!(expense_id, output.expenses.first().unwrap().id.value());

    Ok(())
}

#[tokio::test]
async fn happy_path_multiple_expenses_from_single_group() -> anyhow::Result<()> {
    let db_pool =
        build_test_database(FILE, "happy_path_multiple_expenses_from_single_group").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let bob_id = ctx.users().create_user("Bob").await?;
    let group_id = ctx
        .groups()
        .create_empty_group("Bob's group", bob_id)
        .await?;
    let expense_1_id = ctx
        .expense_entries()
        .create_expense_for_all_group_members(
            group_id,
            bob_id,
            54,
            bob_id,
            "2025-01-01T12:00:00Z".parse()?,
        )
        .await?;
    let expense_2_id = ctx
        .expense_entries()
        .create_expense_for_all_group_members(
            group_id,
            bob_id,
            89,
            bob_id,
            "2025-01-01T08:00:00Z".parse()?,
        )
        .await?;
    let expense_3_id = ctx
        .expense_entries()
        .create_expense_for_all_group_members(
            group_id,
            bob_id,
            112,
            bob_id,
            "2026-03-31T10:30:00Z".parse()?,
        )
        .await?;

    // When
    let output = ctx.expense_entries().get_latest_expenses(bob_id).await?;

    // Then
    assert_eq!(3, output.expenses.len());
    assert_eq!(3, output.total_items);

    let first_expense = output.expenses.first().unwrap();
    let second_expense = output.expenses.get(1).unwrap();
    let third_expense = output.expenses.last().unwrap();

    assert_eq!(expense_3_id, first_expense.id.value());
    assert_eq!(expense_1_id, second_expense.id.value());
    assert_eq!(expense_2_id, third_expense.id.value());

    Ok(())
}

#[tokio::test]
async fn happy_path_multiple_expenses_from_multiple_groups() -> anyhow::Result<()> {
    let db_pool =
        build_test_database(FILE, "happy_path_multiple_expenses_from_multiple_groups").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let bob_id = ctx.users().create_user("Bob").await?;
    let group_1_id = ctx.groups().create_empty_group("group 1", bob_id).await?;
    let group_2_id = ctx.groups().create_empty_group("group 2", bob_id).await?;

    let group_1_expense_1_id = ctx
        .expense_entries()
        .create_expense_for_all_group_members(group_1_id, bob_id, 200, bob_id, Utc::now())
        .await?;

    let group_1_expense_2_id = ctx
        .expense_entries()
        .create_expense_for_all_group_members(group_1_id, bob_id, 71, bob_id, Utc::now())
        .await?;

    let group_2_expense_id = ctx
        .expense_entries()
        .create_expense_for_all_group_members(group_2_id, bob_id, 24, bob_id, Utc::now())
        .await?;

    // When
    let output = ctx.expense_entries().get_latest_expenses(bob_id).await?;

    // Then
    assert_eq!(3, output.expenses.len());
    assert_eq!(3, output.total_items);

    let first_expense = output.expenses.first().unwrap();
    let second_expense = output.expenses.get(1).unwrap();
    let third_expense = output.expenses.last().unwrap();

    assert_eq!(group_2_expense_id, first_expense.id.value());
    assert_eq!(group_1_expense_2_id, second_expense.id.value());
    assert_eq!(group_1_expense_1_id, third_expense.id.value());

    Ok(())
}

#[tokio::test]
async fn happy_path_pagination_works() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "happy_path_pagination_works").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let bob_id = ctx.users().create_user("Bob").await?;
    let group_id = ctx
        .groups()
        .create_empty_group("Bob's group", bob_id)
        .await?;
    ctx.expense_entries()
        .create_expense_for_all_group_members(
            group_id,
            bob_id,
            54,
            bob_id,
            "2025-01-01T12:00:00Z".parse()?,
        )
        .await?;
    let expense_2_id = ctx
        .expense_entries()
        .create_expense_for_all_group_members(
            group_id,
            bob_id,
            54,
            bob_id,
            "2025-01-01T08:00:00Z".parse()?,
        )
        .await?;
    ctx.expense_entries()
        .create_expense_for_all_group_members(
            group_id,
            bob_id,
            54,
            bob_id,
            "2026-03-31T10:30:00Z".parse()?,
        )
        .await?;

    // When
    let output = ctx
        .expense_entries()
        .get_latest_expenses_with_pagination(bob_id, 2, 2)
        .await?;

    // Then
    assert_eq!(1, output.expenses.len());
    assert_eq!(3, output.total_items);

    assert_eq!(expense_2_id, output.expenses.first().unwrap().id.value());

    Ok(())
}

#[tokio::test]
async fn happy_path_multiple_expenses_different_user() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "happy_path_multiple_expenses_different_user").await?;
    let ctx = TestContext::new(db_pool);

    // Given
    let bob_id = ctx.users().create_user("Bob").await?;
    let alice_id = ctx.users().create_user("Alice").await?;

    let group_id = ctx
        .groups()
        .create_empty_group("Bob's group", bob_id)
        .await?;

    ctx.expense_entries()
        .create_expense_for_all_group_members(group_id, bob_id, 54, bob_id, Utc::now())
        .await?;
    ctx.expense_entries()
        .create_expense_for_all_group_members(group_id, bob_id, 37, bob_id, Utc::now())
        .await?;
    ctx.expense_entries()
        .create_expense_for_all_group_members(group_id, bob_id, 89, bob_id, Utc::now())
        .await?;

    // When
    let output = ctx.expense_entries().get_latest_expenses(alice_id).await?;

    // Then
    assert!(output.expenses.is_empty());
    assert_eq!(0, output.total_items);

    Ok(())
}

#[tokio::test]
async fn happy_path_user_expense_participant() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "happy_path_user_expense_participant").await?;
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

    ctx.expense_entries()
        .create_expense_for_all_group_members(bob_group_id, bob_id, 90, bob_id, Utc::now())
        .await?;

    let expense_id = ctx
        .expense_entries()
        .create_expense_for_all_group_members(alice_group_id, alice_id, 78, bob_id, Utc::now())
        .await?;

    // When
    let output = ctx.expense_entries().get_latest_expenses(alice_id).await?;

    // Then
    assert_eq!(1, output.expenses.len());
    assert_eq!(1, output.total_items);

    assert_eq!(expense_id, output.expenses.first().unwrap().id.value());

    Ok(())
}

#[tokio::test]
async fn contains_correct_data() -> anyhow::Result<()> {
    let db_pool = build_test_database(FILE, "contains_correct_data").await?;
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

    let expense_1_id = ctx
        .expense_entries()
        .create_expense_for_all_group_members(
            bob_group_id,
            bob_id,
            90,
            bob_id,
            "2025-01-01T12:00:00Z".parse()?,
        )
        .await?;

    let expense_2_id = ctx
        .expense_entries()
        .create_expense_for_all_group_members(
            alice_group_id,
            alice_id,
            78,
            bob_id,
            "2026-01-31T00:01:00Z".parse()?,
        )
        .await?;

    // When
    let output = ctx.expense_entries().get_latest_expenses(bob_id).await?;

    // Then
    assert_eq!(2, output.expenses.len());
    assert_eq!(2, output.total_items);

    let first = output.expenses.first().unwrap();
    let second = output.expenses.last().unwrap();

    let expected_first = ExpenseSummary {
        id: ExpenseId::new(expense_2_id)?,
        group: GroupSummary {
            id: GroupId::new(alice_group_id)?,
            name: "Alice's group".parse()?,
        },
        occurred_at: "2026-01-31T00:01:00Z".parse()?,
        total: Money::from_euros(78),
        paid_by: UserSummary {
            id: UserId::new(alice_id)?,
            name: "Alice".parse()?,
        },
    };

    let expected_second = ExpenseSummary {
        id: ExpenseId::new(expense_1_id)?,
        group: GroupSummary {
            id: GroupId::new(bob_group_id)?,
            name: "Bob's group".parse()?,
        },
        occurred_at: "2025-01-01T12:00:00Z".parse()?,
        total: Money::from_euros(90),
        paid_by: UserSummary {
            id: UserId::new(bob_id)?,
            name: "Bob".parse()?,
        },
    };

    assert_eq!(&expected_first, first);
    assert_eq!(&expected_second, second);

    Ok(())
}
