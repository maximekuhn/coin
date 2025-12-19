use domain::{
    entities::Group,
    types::{group_id::GroupId, user_id::UserId},
};
use sqlx::{SqlitePool, types::chrono::Utc};

mod fixtures;

// -- exists_by_name_for_owner

macro_rules! exists_by_name_for_owner_test {
    ($scenario:ident, $group_name:expr, $owner_id:expr, $expected:expr) => {
        #[sqlx::test(fixtures("users", "groups"))]
        async fn $scenario(pool: SqlitePool) {
            let name = $group_name;
            let owner_id = $owner_id;

            let mut tx = pool.begin().await.unwrap();
            let exists =
                database::queries::group::exists_by_name_for_owner(&mut tx, &name, &owner_id)
                    .await
                    .unwrap();

            let expected: bool = $expected;
            assert_eq!(expected, exists);
        }
    };
}

exists_by_name_for_owner_test!(
    exists_by_name_for_owner_found,
    fixtures::groups::trip_to_europe_2025().name,
    fixtures::users::johndoe().id,
    true
);

exists_by_name_for_owner_test!(
    exists_by_name_for_owner_groupname_not_existing,
    "Not existing group name".parse().unwrap(),
    fixtures::users::johndoe().id,
    false
);

exists_by_name_for_owner_test!(
    exists_by_name_for_owner_different_owner,
    fixtures::groups::trip_to_europe_2025().name,
    fixtures::users::bill().id,
    false
);

// -- create

#[sqlx::test(fixtures("users", "groups"))]
async fn create_ok(pool: SqlitePool) {
    let group = Group::new(
        GroupId::new_random(),
        "Trip to Paris".parse().unwrap(),
        fixtures::users::marc().id,
        vec![],
        Utc::now(),
    );

    let mut tx = pool.begin().await.unwrap();
    let res = database::queries::group::create(&mut tx, &group).await;
    assert!(res.is_ok());
}

#[sqlx::test(fixtures("users", "groups"))]
async fn create_same_name_different_case_ok(pool: SqlitePool) {
    let name = fixtures::groups::trip_to_europe_2025()
        .name
        .value()
        .to_lowercase()
        .parse()
        .unwrap();
    let group = Group::new(
        GroupId::new_random(),
        name,
        fixtures::groups::trip_to_europe_2025().owner_id,
        vec![],
        Utc::now(),
    );

    let mut tx = pool.begin().await.unwrap();
    let res = database::queries::group::create(&mut tx, &group).await;
    assert!(res.is_ok());
}

#[sqlx::test(fixtures("users", "groups"))]
async fn create_with_owner_and_single_member_ok(pool: SqlitePool) {
    let group = Group::new(
        GroupId::new_random(),
        "Trip to Paris".parse().unwrap(),
        fixtures::users::marc().id,
        vec![fixtures::users::bill().id],
        Utc::now(),
    );

    let mut tx = pool.begin().await.unwrap();
    let res = database::queries::group::create(&mut tx, &group).await;
    assert!(res.is_ok());
}

#[sqlx::test(fixtures("users", "groups"))]
async fn create_with_members_ok(pool: SqlitePool) {
    let group = Group::new(
        GroupId::new_random(),
        "Trip to Paris".parse().unwrap(),
        fixtures::users::marc().id,
        vec![fixtures::users::bill().id, fixtures::users::johndoe().id],
        Utc::now(),
    );

    let mut tx = pool.begin().await.unwrap();
    let res = database::queries::group::create(&mut tx, &group).await;
    assert!(res.is_ok());
}

#[sqlx::test(fixtures("users", "groups"))]
async fn create_with_members_err_unique_violation_same_name_for_owner(pool: SqlitePool) {
    let group = Group::new(
        GroupId::new_random(),
        fixtures::groups::trip_to_europe_2025().name,
        fixtures::groups::trip_to_europe_2025().owner_id,
        vec![fixtures::users::bill().id, fixtures::users::johndoe().id],
        Utc::now(),
    );

    let mut tx = pool.begin().await.unwrap();
    let err = database::queries::group::create(&mut tx, &group)
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

#[sqlx::test(fixtures("users", "groups"))]
async fn create_with_members_err_unique_violation_duplicate(pool: SqlitePool) {
    let mut tx = pool.begin().await.unwrap();
    let err = database::queries::group::create(&mut tx, &fixtures::groups::trip_to_europe_2025())
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

#[sqlx::test(fixtures("users", "groups"))]
async fn create_err_foreign_key_owner_not_existing(pool: SqlitePool) {
    let group = Group::new(
        GroupId::new_random(),
        "New group".parse().unwrap(),
        UserId::new_random(),
        vec![],
        Utc::now(),
    );

    let mut tx = pool.begin().await.unwrap();
    let err = database::queries::group::create(&mut tx, &group)
        .await
        .unwrap_err();
    match err {
        database::Error::SqlxError(error) => {
            assert_eq!(
                sqlx::error::ErrorKind::ForeignKeyViolation,
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

#[sqlx::test(fixtures("users", "groups"))]
async fn create_err_foreign_key_member_not_existing(pool: SqlitePool) {
    let group = Group::new(
        GroupId::new_random(),
        "New group".parse().unwrap(),
        fixtures::users::johndoe().id,
        vec![UserId::new_random()],
        Utc::now(),
    );

    let mut tx = pool.begin().await.unwrap();
    let err = database::queries::group::create(&mut tx, &group)
        .await
        .unwrap_err();
    match err {
        database::Error::SqlxError(error) => {
            assert_eq!(
                sqlx::error::ErrorKind::ForeignKeyViolation,
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
