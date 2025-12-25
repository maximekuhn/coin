use std::collections::HashSet;

use domain::{
    testutils::expense_entry::TestExpenseEntry,
    types::{expense_entry_id::ExpenseEntryId, expense_entry_status::ExpenseEntryStatus},
};
use sqlx::{SqlitePool, types::chrono::Utc};
use uuid::Uuid;

mod fixtures;

// -- create

#[sqlx::test(fixtures("users", "groups", "expense_entries"))]
async fn create_ok(pool: SqlitePool) {
    let expense_entry = TestExpenseEntry::new_valid(
        Uuid::now_v7(),
        Uuid::now_v7(),
        fixtures::groups::trip_to_europe_2025().id.value(),
        fixtures::users::johndoe().id.value(),
        HashSet::<Uuid>::new(),
        ExpenseEntryStatus::Active,
        120,
        fixtures::users::johndoe().id.value(),
        Utc::now(),
        Utc::now(),
    );

    let mut tx = pool.begin().await.unwrap();
    let res = database::queries::expense_entry::create(&mut tx, &expense_entry).await;
    dbg!(&res);
    assert!(res.is_ok());
}

#[sqlx::test(fixtures("users", "groups", "expense_entries"))]
async fn create_with_participants_ok(pool: SqlitePool) {
    let expense_entry = TestExpenseEntry::new_valid(
        Uuid::now_v7(),
        Uuid::now_v7(),
        fixtures::groups::trip_to_europe_2025().id.value(),
        fixtures::users::johndoe().id.value(),
        HashSet::<Uuid>::from_iter(vec![
            fixtures::users::bill().id.value(),
            fixtures::users::marc().id.value(),
        ]),
        ExpenseEntryStatus::Active,
        120,
        fixtures::users::johndoe().id.value(),
        Utc::now(),
        Utc::now(),
    );

    let mut tx = pool.begin().await.unwrap();
    let res = database::queries::expense_entry::create(&mut tx, &expense_entry).await;
    dbg!(&res);
    assert!(res.is_ok());
}

#[sqlx::test(fixtures("users", "groups", "expense_entries"))]
async fn create_err_pk_violation(pool: SqlitePool) {
    let mut tx = pool.begin().await.unwrap();
    let err = database::queries::expense_entry::create(
        &mut tx,
        &fixtures::expense_entries::john_and_bill_shared_shared_expenses_active_expense_entry(),
    )
    .await
    .unwrap_err();
    match err {
        database::Error::SqlxError(error) => {
            assert_eq!(
                sqlx::error::ErrorKind::UniqueViolation,
                error.as_database_error().unwrap().kind()
            );
        }
        unexpected => panic!(
            "{}",
            format!(
                "expected database::error::SqlxError but received {}",
                unexpected
            )
        ),
    };
}

// -- get_by_id

#[sqlx::test(fixtures("users", "groups", "expense_entries"))]
async fn get_by_id_not_found(pool: SqlitePool) {
    let mut tx = pool.begin().await.unwrap();
    let res = database::queries::expense_entry::get_by_id(&mut tx, &ExpenseEntryId::new_random())
        .await
        .unwrap();
    assert_eq!(None, res);
}

#[sqlx::test(fixtures("users", "groups", "expense_entries"))]
async fn get_by_id_found(pool: SqlitePool) {
    let mut tx = pool.begin().await.unwrap();
    let expected =
        fixtures::expense_entries::john_and_bill_shared_shared_expenses_active_expense_entry();
    let actual = database::queries::expense_entry::get_by_id(&mut tx, &expected.id)
        .await
        .unwrap();
    assert_eq!(Some(expected), actual);
}

#[sqlx::test(fixtures("users", "groups", "expense_entries"))]
async fn get_by_id_found_status_inactive(pool: SqlitePool) {
    let mut tx = pool.begin().await.unwrap();
    let expected =
        fixtures::expense_entries::john_and_bill_shared_shared_expenses_overwritten_expense_entry();
    let actual = database::queries::expense_entry::get_by_id(&mut tx, &expected.id)
        .await
        .unwrap();
    assert_eq!(Some(expected), actual);
}
