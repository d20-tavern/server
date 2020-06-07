use tavern_server::pathfinder::character::RaceSubtype;
use std::string::ToString;
use uuid::Uuid;
use nebula_form::{Field, Form};
use tavern_server::db;
use tavern_server::forms::TryFromForm;
use nebula_status::{Status, StatusCode};
use tavern_server::status::{Error, Success};
use tavern_server::db::{Insert, GetById, Update, Delete};
use diesel::result::{Error as DieselError, DatabaseErrorKind};
use tavern_server::api::Filters;
use tavern_server::pathfinder::CharacterStat::Race;

const VALID_FIELD_NAME: &str = "foo subtype";
const VALID_FIELD_DESCRIPTION: &str = "a description\nwith\nnewlines";

#[cfg(test)]
fn valid_form() -> Form {
    let mut form = Form::new();
    form.insert(RaceSubtype::FIELD_NAME, Field::Text(VALID_FIELD_NAME.to_string()));
    form.insert(RaceSubtype::FIELD_DESCRIPTION, Field::Text(VALID_FIELD_DESCRIPTION.to_string()));
    form
}

#[cfg(test)]
fn invalid_forms() -> Vec<Form> {
    let mut form = Form::new();
    form.insert(RaceSubtype::FIELD_NAME, Field::Text(String::from("")));
    form.insert(RaceSubtype::FIELD_DESCRIPTION, Field::Text(VALID_FIELD_DESCRIPTION.to_string()));

    vec![form]
}

fn valid_instance_1() -> RaceSubtype {
    RaceSubtype {
        id: Uuid::new_v4(),
        name: VALID_FIELD_NAME.to_string(),
        description: VALID_FIELD_DESCRIPTION.to_string(),
    }
}

fn valid_instance_2() -> RaceSubtype {
    RaceSubtype {
        id: Uuid::new_v4(),
        name: VALID_FIELD_NAME.to_string(),
        description: VALID_FIELD_NAME.to_string(),
    }
}

#[tavern_derive::db_test]
async fn test_race_subtype_from_valid_form() {
    let form = valid_form();
    let instance = valid_instance_1();
    let conn = db::get_connection().await
        .expect("failed to get database connection");

    let result = RaceSubtype::try_from_form(&conn, form, None, None)
        .expect("try_from_form should succeed with valid values");

    assert_eq!(instance.name, result.name);
    assert_eq!(instance.description, result.description);
}

#[tavern_derive::db_test]
async fn test_existing_race_subtype_from_valid_form() {
    let form = valid_form();
    let instance = valid_instance_1();
    let conn = db::get_connection().await
        .expect("failed to get database connection");

    instance.db_insert(&conn)
        .expect("database insertion should not fail");

    let result = RaceSubtype::try_from_form(&conn, form, Some(instance.id.clone()), None)
        .expect("try_from_form should succeed with valid values");

    assert_eq!(instance, result);
}

#[tavern_derive::db_test]
async fn test_race_subtype_from_invalid_form_fails() {
    let forms = invalid_forms();
    let conn = db::get_connection().await
        .expect("failed to get database connection");

    for form in forms.into_iter() {
        let result = RaceSubtype::try_from_form(&conn, form, None, None)
            .expect_err("try_from_form should not succeed with invalid values");

        let status = Status::<Error>::recover(result)
            .expect("should be able to recover a Status<Error> from Rejection");

        assert_eq!(status.code(), &StatusCode::BAD_REQUEST);
    }
}

#[tavern_derive::db_test]
async fn test_insert_subtype_into_and_get_from_database_succeeds() {
    /// Combining both because there is no good way to access the generated
    /// schema crate from the tests/ directory.

    let instance = valid_instance_1();
    let conn = db::get_connection().await
        .expect("failed to get database connection");

    instance.db_insert(&conn)
        .expect("inserting valid data into database should not fail");

    let inserted = RaceSubtype::db_get_by_id(&instance.id, &conn)
        .expect("getting inserted data from database should not fail");

    assert_eq!(inserted, instance);
}

#[tavern_derive::db_test]
async fn test_double_insert_with_different_id_fails() {
    /// Combining both because there is no good way to access the generated
    /// schema crate from the tests/ directory.

    let instance = valid_instance_1();
    let conn = db::get_connection().await
        .expect("failed to get database connection");

    instance.db_insert(&conn)
        .expect("inserting valid data into database should not fail");

    let instance2 = valid_instance_1();

    assert_ne!(instance.id, instance2.id, "random ids should not be equal");

    let err = instance2.db_insert(&conn)
        .expect_err("double insert with different id should fail because names are equal");

    if let db::Error::RunQuery(db_err) = &err {
        if let DieselError::DatabaseError(kind, info) = db_err {
            if let DatabaseErrorKind::UniqueViolation = kind {
                assert!(true);
            } else {
                panic!("unexpected error: {}", db_err);
            }
        } else {
            panic!("unexpected error: {}", db_err);
        }
    } else {
        panic!("unexpected error: {}", err);
    }
}

#[tavern_derive::db_test]
async fn test_update_subtype_into_database_succeeds() {
    let mut instance = valid_instance_1();
    let conn = db::get_connection().await
        .expect("failed to get database connection");

    instance.db_insert(&conn)
        .expect("inserting valid data into database should not fail");

    instance.description.push_str(&VALID_FIELD_DESCRIPTION);

    instance.db_update(&conn)
        .expect("updating valid data into database should not fail");

    let inserted = RaceSubtype::db_get_by_id(&instance.id, &conn)
        .expect("getting inserted data from database should not fail");

    assert_eq!(inserted, instance);
}

#[tavern_derive::db_test]
async fn test_delete_subtype_from_database_succeeds() {
    let mut instance = valid_instance_1();
    let conn = db::get_connection().await
        .expect("failed to get database connection");

    instance.db_insert(&conn)
        .expect("inserting valid data into database should not fail");

    let inserted = RaceSubtype::db_get_by_id(&instance.id, &conn)
        .expect("getting inserted data from database should not fail");

    assert_eq!(inserted, instance);

    instance.db_delete(&conn)
        .expect("deleting inserted data from database should not fail");
}

#[tavern_derive::db_test]
async fn test_double_delete_subtype_from_database_does_not_fail() {
    let mut instance = valid_instance_1();
    let conn = db::get_connection().await
        .expect("failed to get database connection");

    instance.db_insert(&conn)
        .expect("inserting valid data into database should not fail");

    let inserted = RaceSubtype::db_get_by_id(&instance.id, &conn)
        .expect("getting inserted data from database should not fail");

    assert_eq!(inserted, instance);

    instance.db_delete(&conn)
        .expect("deleting inserted data from database should not fail");

    instance.db_delete(&conn)
        .expect("deleting inserted data from database should not fail");
}

#[tavern_derive::db_test]
async fn test_subtype_api_get_by_id() {
    let instance = valid_instance_1();

    let conn =db::get_connection()
        .await
        .expect("failed to get database connection");

    instance.db_insert(&conn)
        .expect("database insertion should not fail");

    let reply = warp::test::request()
        .path(&format!("/race-subtypes/{}", instance.id))
        .method("GET")
        .reply(&RaceSubtype::filters(None))
        .await;

    let success: Success<RaceSubtype> = serde_json::from_slice(reply.body().as_ref())
        .expect("expected to read a Success<RaceSubtype> from the reply");

    assert_eq!(instance, success.data);
}

#[tavern_derive::db_test]
async fn test_subtype_api_get_all() {
    let instance = valid_instance_1();

    let conn =db::get_connection()
        .await
        .expect("failed to get database connection");

    instance.db_insert(&conn)
        .expect("database insertion should not fail");

    let mut instance2 = valid_instance_1();

    instance2.name.push_str(VALID_FIELD_NAME);

    instance2.db_insert(&conn)
        .expect("database insertion should not fail");

    let reply = warp::test::request()
        .path("/race-subtypes")
        .method("GET")
        .reply(&RaceSubtype::filters(None))
        .await;

    let success: Success<Vec<RaceSubtype>> = serde_json::from_slice(reply.body().as_ref())
        .expect("expected to read a Success<Vec<RaceSubtype>> from the reply");

    assert_eq!(success.data.len(), 2);

    let result1 = success.data.get(0).unwrap();
    let result2 = success.data.get(1).unwrap();

    // This assumes that the results are sorted by id
    if instance.id < instance2.id {
        assert_eq!(&instance, result1);
        assert_eq!(&instance2, result2);
    } else {
        assert_eq!(&instance2, result1);
        assert_eq!(&instance, result2);
    }
}

#[tavern_derive::db_test]
async fn test_subtype_api_insert() {
    let form = valid_form();

    let reply = warp::test::request()
        .path("/race-subtypes")
        .method("POST")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(form.to_url_encoded().unwrap())
        .reply(&RaceSubtype::filters(None))
        .await;

    let success : Success<RaceSubtype> = serde_json::from_slice(reply.body().as_ref())
        .expect("expected to read a Success<RaceSubtype> from insert reply");

    let conn = db::get_connection().await
        .expect("failed to get database connection");

    let instance = RaceSubtype::db_get_by_id(&success.data.id, &conn)
        .expect("should be able to get inserted item by id");

    assert_eq!(success.data, instance);
}

#[tavern_derive::db_test]
async fn test_subtype_api_update() {
    let conn = db::get_connection().await
        .expect("failed to get database connection");

    let instance1 = valid_instance_1();

    instance1.db_insert(&conn)
        .expect("database insertion should not fail");

    let instance2 = valid_instance_2();

    assert_ne!(instance1, instance2);

    let mut form = valid_form();

    form.insert(RaceSubtype::FIELD_NAME, Field::Text(instance2.name.clone()));
    form.insert(RaceSubtype::FIELD_DESCRIPTION, Field::Text(instance2.description.clone()));

    warp::test::request()
        .path(&format!("/race-subtypes/{}", instance1.id))
        .method("PUT")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(form.to_url_encoded().unwrap())
        .filter(&RaceSubtype::filters(None))
        .await
        .expect("mocked PUT request should not fail");

    let result = RaceSubtype::db_get_by_id(&instance1.id, &conn)
        .expect("getting updated item from database should succeed");

    assert_eq!(result.id, instance1.id);
    assert_eq!(result.name, instance2.name);
    assert_eq!(result.description, instance2.description);
}

#[tavern_derive::db_test]
async fn test_subtype_api_delete() {
    let conn = db::get_connection().await
        .expect("failed to get database connection");

    let instance = valid_instance_1();

    warp::test::request()
        .path(&format!("/race-subtypes/{}", instance.id))
        .method("DELETE")
        .filter(&RaceSubtype::filters(None))
        .await
        .expect("mocked DELETE request should not fail");

    let result = RaceSubtype::db_get_by_id(&instance.id, &conn)
        .expect_err("getting delete item from database should fail");

    match result {
        db::Error::RunQuery(db_err) => {
            match db_err {
                DieselError::NotFound => {},
                err => panic!("unexpected error: {}", err),
            }
        },
        err => panic!("unexpected error: {}", err),
    }
}