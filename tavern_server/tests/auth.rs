use nebula_form::{Form, Field};
use tavern_server::auth;

const TEST_USERNAME: &'static str = "tavern_test_user";
const TEST_PASSWORD: &'static str = "hunter2:super$3cure";
const TEST_EMAIL: &'static str = "username@example.com";

fn get_registration_form() -> Form {
    let mut form = Form::new();

    form.insert(auth::FIELD_EMAIL, Field::Text(TEST_EMAIL.to_string()));
    form.insert(auth::FIELD_USERNAME, Field::Text(TEST_USERNAME.to_string()));
    form.insert(auth::FIELD_PASSWORD, Field::Text(TEST_PASSWORD.to_string()));

    form
}

fn get_login_form() -> Form {
    let mut form = Form::new();

    form.insert(auth::FIELD_USERNAME, Field::Text(TEST_USERNAME.to_string()));
    form.insert(auth::FIELD_PASSWORD, Field::Text(TEST_PASSWORD.to_string()));

    form
}

#[tavern_derive::db_test]
async fn registration_updates_database() {
    let form = get_registration_form();
    let resp = warp::test::request()
        .path("/register")
        .method("POST")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(form.to_url_encoded().unwrap().as_bytes())
        .filter(&auth::register_filter())
        .await
        .unwrap();
}

#[tavern_derive::db_test]
async fn double_registration_fails() {

}

#[tavern_derive::db_test]
async fn login_no_such_user_fails() {

}

#[tavern_derive::db_test]
async fn login_wrong_credentials_fails() {

}

#[tavern_derive::db_test]
async fn valid_login_succeeds() {
        let data = base64::encode(format!("{}:{}", username, password));
        let resp = warp::test::request()
            .method("POST")
            .header(http::header::AUTHORIZATION, format!("Basic {}", data))

}

#[tavern_derive::db_test]
async fn correct_user_on_login() {

}
