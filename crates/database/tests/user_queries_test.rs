use domain::{testutils::user::TestUser, types::role::Role};
use email_address::EmailAddress;
use sqlx::{SqlitePool, types::chrono::Utc};
use uuid::Uuid;

// -- email_exists

#[sqlx::test]
async fn email_exists_false(pool: SqlitePool) {
    let email = EmailAddress::new_unchecked("john.doe@gmail.com");
    let mut tx = pool.begin().await.unwrap();
    let exists = database::queries::user::email_exists(&mut tx, &email)
        .await
        .unwrap();
    assert!(!exists);
}

#[sqlx::test(fixtures("users"))]
async fn email_exists_with_fixtures_true(pool: SqlitePool) {
    let email = EmailAddress::new_unchecked("john.doe@gmail.com");
    let mut tx = pool.begin().await.unwrap();
    let exists = database::queries::user::email_exists(&mut tx, &email)
        .await
        .unwrap();
    assert!(exists);
}

#[sqlx::test(fixtures("users"))]
async fn email_exists_with_fixtures_false(pool: SqlitePool) {
    let email = EmailAddress::new_unchecked("yannick.noah@gmail.com");
    let mut tx = pool.begin().await.unwrap();
    let exists = database::queries::user::email_exists(&mut tx, &email)
        .await
        .unwrap();
    assert!(!exists);
}

// -- create_user

#[sqlx::test]
async fn create_ok(pool: SqlitePool) {
    let user = TestUser::new_valid(
        Uuid::now_v7(),
        "John",
        "john.doe@gmail.com",
        Role::User,
        Utc::now(),
    );

    let mut tx = pool.begin().await.unwrap();
    let res = database::queries::user::create(&mut tx, &user).await;
    assert!(res.is_ok());
}

#[sqlx::test(fixtures("users"))]
async fn create_with_fixtures_ok(pool: SqlitePool) {
    let user = TestUser::new_valid(
        Uuid::now_v7(),
        "Bob",
        "bob@gmail.com",
        Role::User,
        Utc::now(),
    );

    let mut tx = pool.begin().await.unwrap();
    let res = database::queries::user::create(&mut tx, &user).await;
    assert!(res.is_ok());
}

#[sqlx::test(fixtures("users"))]
async fn create_with_fixtures_same_username_ok(pool: SqlitePool) {
    let user = TestUser::new_valid(
        Uuid::now_v7(),
        "johndoe",
        "bob@gmail.com",
        Role::User,
        Utc::now(),
    );

    let mut tx = pool.begin().await.unwrap();
    let res = database::queries::user::create(&mut tx, &user).await;
    assert!(res.is_ok());
}

#[sqlx::test(fixtures("users"))]
async fn create_with_fixtures_err_unique_violation(pool: SqlitePool) {
    let user = TestUser::new_valid(
        Uuid::now_v7(),
        "John",
        "john.doe@gmail.com",
        Role::User,
        Utc::now(),
    );

    let mut tx = pool.begin().await.unwrap();
    let err = database::queries::user::create(&mut tx, &user)
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
