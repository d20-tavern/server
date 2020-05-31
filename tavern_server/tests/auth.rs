use http::header;
use nebula_form::{Field, Form};
use nebula_status::{Empty, Status, StatusCode};
use tavern_server::auth;
use tavern_server::status::{Error, Success};
use warp::reject::Rejection;

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

async fn registration_request(form: &Form) -> Result<Status<Success<auth::User>>, Rejection> {
    warp::test::request()
        .path("/register")
        .method("POST")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(&form.to_url_encoded().unwrap().as_bytes())
        .filter(&auth::register_filter())
        .await
}

async fn login_request(user: &str, pass: &str) -> Result<Status<Empty>, Rejection> {
    let data = base64::encode(format!("{}:{}", user, pass));
    warp::test::request()
        .path("/login")
        .method("GET")
        .header(http::header::AUTHORIZATION, format!("Basic {}", data))
        .filter(&auth::login_filter())
        .await
}

#[tavern_derive::db_test]
async fn registration_updates_database() {
    let form = get_registration_form();
    let resp = registration_request(&form)
        .await
        .expect("single registration should succeed");

    let user = &resp.data().expect("registration should return a User").data;

    assert_eq!(user.username, TEST_USERNAME);
    assert_eq!(user.email, TEST_EMAIL);
    assert_eq!(user.is_admin, false);
}

#[tavern_derive::db_test]
async fn double_registration_fails() {
    let form = get_registration_form();
    // We only care that this succeeds
    let _ = registration_request(&form)
        .await
        .expect("single registration should succeed");

    let resp = registration_request(&form)
        .await
        .expect_err("double registration should not succeed");

    let stat: Status<Error> = Status::recover(resp).expect("Rejection should contain a Status");

    eprintln!("{:?}", stat);
    assert_eq!(
        stat.code(),
        &StatusCode::BAD_REQUEST,
        "double registration should fail with 400 Bad Request"
    );
}

#[tavern_derive::db_test]
async fn login_no_such_user_fails() {
    let resp = login_request(TEST_USERNAME, TEST_PASSWORD)
        .await
        .expect_err("login without registration should fail");

    let stat: Status<Empty> =
        Status::recover(resp).expect("Rejection should contain a Status<Empty>");

    assert_eq!(stat.code(), &StatusCode::UNAUTHORIZED);
    let val = stat
        .headers()
        .get(header::WWW_AUTHENTICATE)
        .expect("WWW-Authenticate header must exist");

    assert!(val.to_str().unwrap().contains("Basic"));
}

#[tavern_derive::db_test]
async fn login_wrong_credentials_fails() {
    let form = get_registration_form();

    let _ = registration_request(&form)
        .await
        .expect("single registration should succeed");

    // Ensure the username and password are not the same for the test
    assert_ne!(TEST_USERNAME, TEST_PASSWORD);
    let resp = login_request(TEST_USERNAME, TEST_USERNAME)
        .await
        .expect_err("invalid login for valid user should fail");

    let stat: Status<Empty> =
        Status::recover(resp).expect("Rejection should contain a Status<Empty>");

    assert_eq!(stat.code(), &StatusCode::UNAUTHORIZED);
    stat.headers()
        .get(header::WWW_AUTHENTICATE)
        .expect("WWW-Authenticate header must exist");
}

#[tavern_derive::db_test]
async fn valid_login_succeeds() {
    let form = get_registration_form();

    let _ = registration_request(&form)
        .await
        .expect("single registration should succeed");

    // Ensure the username and password are not the same for the test
    let resp: Status<Empty> = login_request(TEST_USERNAME, TEST_PASSWORD)
        .await
        .expect("valid login for valid user should succeed");

    assert_eq!(resp.code(), &StatusCode::OK);
}

#[tavern_derive::db_test]
async fn correct_user_on_login() {}
