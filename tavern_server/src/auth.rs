use crate::{forms, schema};
use crate::status::{Error, Success};
use crate::{config, db, status};
use argon2::{self, Config, ThreadMode, Variant, Version};
use bytes::Bytes;
use http::HeaderValue;
use nebula_form::Form;
use nebula_status::{Empty, Status, StatusCode};
use rand::RngCore;
use serde::Serialize;
use std::convert::TryFrom;
use structopt::StructOpt;
use warp::filters::BoxedFilter;
use warp::reject::Rejection;
use warp::Filter;
use uuid::Uuid;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::result::Error as DieselError;
use diesel::result::DatabaseErrorKind;
use crate::db::{GetAll, GetById, Insert, Update, Delete};

use crate::schema::users;

/// The length of an Argon2i hash, in bytes.
pub const ARGON2_HASH_LENGTH: u32 = 32;
/// The length of a generated salt, in bytes.
pub const ARGON2_SALT_LENGTH: usize = 32;

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;
    use nebula_form::{Field, Form};

    const TEST_MEMORY: u32 = 1024u32;
    const TEST_TIME_COST: u32 = 10u32;
    const TEST_THREADS: u32 = 4_u32;

    #[test]
    fn argon2_config_memory_is_set() {
        let a2 = Argon2Opt {
            memory: TEST_MEMORY,
            time_cost: TEST_TIME_COST,
            threads: TEST_THREADS,
        };

        let a2conf: Config = a2.into();

        assert_eq!(a2conf.mem_cost, TEST_MEMORY);
    }

    #[test]
    fn argon2_config_time_cost_is_set() {
        let a2 = Argon2Opt {
            memory: TEST_MEMORY,
            time_cost: TEST_TIME_COST,
            threads: TEST_THREADS,
        };

        let a2conf: Config = a2.into();

        assert_eq!(a2conf.time_cost, TEST_TIME_COST);
    }

    #[test]
    fn argon2_config_threads_is_set() {
        let a2 = Argon2Opt {
            memory: TEST_MEMORY,
            time_cost: TEST_TIME_COST,
            threads: TEST_THREADS,
        };

        let a2conf: Config = a2.into();

        assert_eq!(a2conf.lanes, TEST_THREADS);
    }

    #[test]
    fn from_form_to_registration_info_succeeds() {
        let user = "username";
        let pass = "hunter2";
        let email = "user@domain.com";

        let mut form = Form::with_capacity(2);
        form.insert(FIELD_USERNAME, Field::Text(user.to_string()));
        form.insert(FIELD_PASSWORD, Field::Text(pass.to_string()));
        form.insert(FIELD_EMAIL, Field::Text(email.to_string()));

        let info = RegistrationInfo::try_from(form).unwrap();
        assert_eq!(info.user.username, user);
        assert_eq!(info.password, pass);
        assert_eq!(info.user.email, email);
    }

    #[test]
    fn hash_succeeds() {
        let salt = b"super secret salt";
        let pass = b"p@ssw0rd";
        let conf = argon2::Config::default();

        let expected = argon2::hash_raw(pass, salt, &conf).unwrap();

        let hash = block_on(hash_password(pass, salt, &conf)).unwrap();

        // Note: for actual application uses, argon2::verify_raw should be used instead
        assert_eq!(expected, hash);
    }

    #[tokio::test]
    async fn retrieves_registration_info() {
        // get_registration_info()
        let username = "foobar";
        let password = "hunter2";
        let email = "email@domain.org";

        let mut form = Form::with_capacity(3);
        form.insert(FIELD_USERNAME, Field::Text(username.to_string()));
        form.insert(FIELD_PASSWORD, Field::Text(password.to_string()));
        form.insert(FIELD_EMAIL, Field::Text(email.to_string()));

        let info = warp::test::request()
            .method("POST")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form.to_url_encoded().unwrap().as_bytes())
            .filter(&get_registration_info())
            .await
            .unwrap();

        assert_eq!(info.user.username, username);
        assert_eq!(info.user.email, email);
        assert_eq!(info.password, password);
        assert!(!info.user.is_admin);
    }

    #[tokio::test]
    async fn retrieves_registration_info_ignores_user_id() {
        let username = "foobar";
        let password = "hunter2";
        let email = "email@domain.org";
        let id = Uuid::new_v4().to_hyphenated();

        let mut form = Form::with_capacity(3);
        form.insert(FIELD_USERNAME, Field::Text(username.to_string()));
        form.insert(FIELD_PASSWORD, Field::Text(password.to_string()));
        form.insert(FIELD_EMAIL, Field::Text(email.to_string()));
        form.insert(FIELD_USER_ID, Field::Text(id.to_string()));

        let info = warp::test::request()
            .method("POST")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form.to_url_encoded().unwrap().as_bytes())
            .filter(&get_registration_info())
            .await
            .unwrap();

        assert_eq!(info.user.id, None);
    }

    #[tokio::test]
    async fn retrieves_registration_info_ignores_is_admin() {
        let username = "foobar";
        let password = "hunter2";
        let email = "email@domain.org";

        let mut form = Form::with_capacity(3);
        form.insert(FIELD_USERNAME, Field::Text(username.to_string()));
        form.insert(FIELD_PASSWORD, Field::Text(password.to_string()));
        form.insert(FIELD_EMAIL, Field::Text(email.to_string()));
        form.insert(FIELD_IS_ADMIN, Field::Text("false".to_string()));

        let info = warp::test::request()
            .method("POST")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form.to_url_encoded().unwrap().as_bytes())
            .filter(&get_registration_info())
            .await
            .unwrap();

        assert!(!info.user.is_admin);
    }

    #[tokio::test]
    async fn registration_to_user_and_user_auth() {
        let pass = "hunter2";
        let exp_user = User {
            id: None,
            username: "foobar".to_string(),
            email: "example@domain.org".to_string(),
            is_admin: false,
        };
        let info = RegistrationInfo {
            user: exp_user.clone(),
            password: pass.to_string(),
        };
        let salt = b"super secret salt".to_vec();
        // TODO: Make sure it works for non-default config
        let conf = argon2::Config::default();

        let expected_hash = argon2::hash_raw(pass.as_bytes(), salt.as_slice(), &conf).unwrap();

        let (user, auth) = registration_to_user_auth(info, salt.clone(), conf.clone())
            .await
            .unwrap();

        assert_eq!(exp_user, user);
        assert_eq!(expected_hash, auth.hash);
        assert_eq!(salt, auth.salt.as_slice());
        assert_eq!(conf, auth.config);
    }

    #[tokio::test]
    async fn get_credentials_from_header() {
        let username = "justsomeone";
        let password = "this:contains:colons";
        let data = base64::encode(format!("{}:{}", username, password));
        let resp = warp::test::request()
            .method("POST")
            .header(http::header::AUTHORIZATION, format!("Basic {}", data))
            .filter(&credentials_from_header())
            .await
            .unwrap();

        let (u, p) = resp;
        assert_eq!(&u, username);
        assert_eq!(&p, password);
    }
}

/// An application-specific representation of Argon2i configuration,
/// to make it easier to read related configurations from the ENV,
/// command-line and the database.
#[derive(StructOpt, Clone, Debug)]
pub struct Argon2Opt {
    #[structopt(
        long = "argon-memory",
        env = "TAVERN_ARGON2_MEMORY",
        help = "the amount of memory in KB to use while hashing"
    )]
    memory: u32,
    #[structopt(
        long = "argon-time-cost",
        env = "TAVERN_ARGON2_TIME_COST",
        help = "the amount of time a single hash should take"
    )]
    time_cost: u32,
    #[structopt(
        long = "argon-threads",
        env = "TAVERN_ARGON2_THREADS",
        help = "the number of threads to use while hashing"
    )]
    threads: u32,
}

impl From<Argon2Opt> for argon2::Config<'static> {
    fn from(opt: Argon2Opt) -> Config<'static> {
        let mut config = Config::default();
        config.hash_length = ARGON2_HASH_LENGTH;
        config.variant = Variant::Argon2i;
        config.version = Version::Version13;
        config.thread_mode = ThreadMode::Parallel;
        config.mem_cost = opt.memory;
        config.lanes = opt.threads;
        config.time_cost = opt.time_cost;
        config
    }
}

/// The name of the user email's UNIQUE column constraint
const CONSTRAINT_USER_EMAIL_UNIQUE: &str = "user_email_unique";
/// The name of the user email's UNIQUE column constraint
const CONSTRAINT_USER_USERNAME_UNIQUE: &str = "user_username_unique";

/// The expected form field name for the user ID.
pub const FIELD_USER_ID: &str = "user-id";
/// The expected form field name for the user's email.
pub const FIELD_EMAIL: &str = "email";
/// The expected form field name for whether the user is an admin
/// or not (ignored in certain contexts).
pub const FIELD_IS_ADMIN: &str = "is-admin";
/// The expected form field name for the user's password.
pub const FIELD_PASSWORD: &str = "password";
/// The expected form field name for the user's username.
pub const FIELD_USERNAME: &str = "username";

/// Represents a user of the application.
#[derive(Serialize, Clone, Debug)]
pub struct User {
    /// The User's unique ID. If None, this user is being registered
    /// or is invalid.
    pub id: Option<Uuid>,
    /// The User's username.
    pub username: String,
    /// The User's email address.
    pub email: String,
    /// Whether the User is an admin or not.
    pub is_admin: bool,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        if self.id.is_some() && other.id.is_some() {
            self.id.unwrap() == other.id.unwrap()
        } else {
            self.username == other.username || self.email == other.email
        }
    }
}

impl Eq for User {}

impl TryFrom<Form> for User {
    type Error = Rejection;

    fn try_from(form: Form) -> Result<Self, Self::Error> {
        let username = forms::get_form_text_field(&form, FIELD_USERNAME)?;
        let email = forms::get_form_text_field(&form, FIELD_EMAIL)?;
        let is_admin = form
            .get(FIELD_IS_ADMIN)
            .map(|field| {
                field
                    .as_text()
                    .map(|val| -> Result<bool, Rejection> { Ok(val.eq_ignore_ascii_case("true")) })
                    .ok_or_else(|| forms::field_is_file_error(FIELD_IS_ADMIN))?
            })
            .unwrap_or(Ok(false))?;
        let id = form.get(FIELD_USER_ID).map(|field| {
            field
                .as_text()
                .map(|val| {
                    Uuid::parse_str(val).map_err(|_| forms::field_is_invalid_error(FIELD_USER_ID))
                })
                .ok_or_else(|| forms::field_is_file_error(FIELD_USER_ID))?
        });

        // Not sure how else to extract the Result from an Option
        let id = match id {
            None => None,
            Some(val) => Some(val?),
        };

        Ok(User {
            id,
            username,
            email,
            is_admin,
        })
    }
}

impl From<User> for Bytes {
    fn from(user: User) -> Self {
        status::serialize_to_bytes(&user)
    }
}

/// Represents a User's authentication parameters. Used to help authenticate
/// the user without passing this information around to the rest of the
/// program.
struct UserAuth {
    hash: Vec<u8>,
    salt: Vec<u8>,
    config: argon2::Config<'static>,
}

impl UserAuth {
    fn is_valid(&self, pass: &str) -> Result<bool, Rejection> {
        argon2::verify_raw(pass.as_bytes(), &self.salt, &self.hash, &self.config)
            .map_err(|err| status::server_error_into_rejection(err.to_string()))
    }
}

#[derive(AsChangeset, Associations, Identifiable, Insertable, Queryable)]
#[derive(GetAll, GetById, Insert, Update, Delete)]
#[table_name = "users"]
struct DBUser {
    id: Uuid,
    email: String,
    username: String,
    is_admin: bool,
    pass_hash: Vec<u8>,
    salt: Vec<u8>,
    time_cost: i32,
    memory: i32,
    threads: i32,
}

impl From<(User, UserAuth)> for DBUser {
    fn from((user, auth): (User, UserAuth)) -> Self {
        DBUser {
            // TODO: fix User to not use Option
            id: user.id.unwrap(),
            email: user.email,
            username: user.username,
            is_admin: user.is_admin,
            pass_hash: auth.hash,
            salt: auth.salt,
            time_cost: auth.config.time_cost as i32,
            memory: auth.config.mem_cost as i32,
            threads: auth.config.lanes as i32,
        }
    }
}

impl Into<(User, UserAuth)> for DBUser {
    fn into(self) -> (User, UserAuth) {
        let user = User {
            id: Some(self.id),
            username: self.username,
            email: self.email,
            is_admin: self.is_admin
        };

        let config = Argon2Opt {
            memory: self.memory as u32,
            time_cost: self.time_cost as u32,
            threads: self.threads as u32,
        };

        let auth = UserAuth {
            hash: self.pass_hash,
            salt: self.salt,
            config: config.into(),
        };

        (user, auth)
    }
}

impl DBUser {
    fn db_from_username(user_username: String, conn: &db::Connection) -> Result<Self, Rejection> {
        use schema::users::dsl::*;
        users.filter(username.eq(&user_username))
            .first::<DBUser>(conn)
            .map_err(|err|
                match err {
                    DieselError::NotFound => reject_login_required(),
                    _ => status::server_error_into_rejection(err.to_string()),
                }
            )
    }

    fn db_from_email(user_email: String, conn: &db::Connection) -> Result<Self, Rejection> {
        use schema::users::dsl::*;
        users.filter(email.eq(&user_email))
            .first::<DBUser>(conn)
            .map_err(|err|
                match err {
                    DieselError::NotFound => reject_login_required(),
                    _ => status::server_error_into_rejection(err.to_string()),
                }
            )
    }
}

/// Contains all of the necessary information related to registering a
/// user.
struct RegistrationInfo {
    user: User,
    password: String,
}

impl TryFrom<Form> for RegistrationInfo {
    type Error = Rejection;
    fn try_from(form: Form) -> Result<Self, Self::Error> {
        let password = forms::get_form_text_field(&form, FIELD_PASSWORD)?;

        let info = RegistrationInfo {
            user: User::try_from(form)?,
            password,
        };

        Ok(info)
    }
}

/// A warp Filter to randomly generate a salt for a user.
fn generate_salt() -> BoxedFilter<(Vec<u8>,)> {
    warp::any()
        .and_then(|| async move {
            let mut salt = [0u8; ARGON2_SALT_LENGTH];
            rand::thread_rng()
                .try_fill_bytes(&mut salt[..])
                .map(|_| salt.to_vec())
                .map_err(|err| {
                    Status::with_message(&StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err))
                })
                .map_err(Rejection::from)
        })
        .boxed()
}

/// A warp Filter that receives the submitted registration form and parses
/// it to extract the required registration information.
fn get_registration_info() -> BoxedFilter<(RegistrationInfo,)> {
    warp::filters::method::post()
        .and(nebula_form::form_filter())
        .and_then(|form: Form| async move {
            RegistrationInfo::try_from(form).map(|mut info| {
                info.user.id = None;
                info
            })
        })
        .boxed()
}

/// A warp Filter that provides a copy of the server's current Argon2
/// configuration.
fn get_argon2_config() -> BoxedFilter<(argon2::Config<'static>,)> {
    config::filter()
        .map(|conf: &'static config::Config| conf.argon2.clone().into())
        .boxed()
}

/// Asynchronously hashes the given password with the given salt and =
/// configuration. Primarily a wrapper to convert errors into ones the server
/// can use.
async fn hash_password(
    password: &[u8],
    salt: &[u8],
    config: &argon2::Config<'static>,
) -> Result<Vec<u8>, Rejection> {
    argon2::hash_raw(password, salt, config)
        .map_err(|err| status::server_error_into_rejection(err.to_string()))
}

/// Takes the given User and UserAuth information, and uses the provided
/// database connection to insert the new user into the database. Attempts to
/// differentiate server errors from client errors (i.e. using a username/email
/// that is already taken).
async fn register_in_database(
    user: User,
    auth: UserAuth,
    conn: db::Connection,
) -> Result<Status<Empty>, Rejection> {
    use schema::users::dsl::*;
    let db_user = DBUser::from((user, auth));
    diesel::insert_into(users)
        .values(db_user)
        .execute(&conn)
        .map(|_| Status::new(&StatusCode::OK))
        .map_err(|err| {
            match &err {
                DieselError::DatabaseError(kind, info) => {
                    match kind {
                        DatabaseErrorKind::UniqueViolation => {
                            match info.constraint_name() {
                                None => Status::with_message(
                                    &StatusCode::BAD_REQUEST,
                                    "a user with that information already exists".to_string()
                                ).into(),
                                Some(name) => match name {
                                    "user_email_unique" => Status::with_message(
                                        &StatusCode::BAD_REQUEST,
                                        "a user with that email already exists".to_string()
                                    ).into(),
                                    "user_username_unique" => Status::with_message(
                                        &StatusCode::BAD_REQUEST,
                                        "a user with that username already exists".to_string()
                                    ).into(),
                                    other => status::server_error_into_rejection(err.to_string()),
                                }
                            }
                        },
                        _ => status::server_error_into_rejection(err.to_string()),
                    }
                },
                other => status::server_error_into_rejection(other.to_string()),
            }
        })
}

/// Asynchronously convert a RegistrationInfo into a User and a UserAuth.
async fn registration_to_user_auth(
    info: RegistrationInfo,
    salt: Vec<u8>,
    config: argon2::Config<'static>,
) -> Result<(User, UserAuth), Rejection> {
    let hash = hash_password(&info.password.as_bytes(), salt.as_slice(), &config).await?;
    let user_auth = UserAuth { hash, salt, config };
    Ok((info.user, user_auth))
}

/// A warp Filter containing the full registration endpoint.
pub fn register_filter() -> BoxedFilter<(Status<Empty>,)> {
    get_registration_info()
        .and(generate_salt())
        .and(get_argon2_config())
        .and_then(registration_to_user_auth)
        .untuple_one()
        .and(db::conn_filter())
        .and_then(register_in_database)
        .boxed()
}

/// Generate a Rejection that tells the client that authentication is required
/// for this resource.
fn reject_login_required() -> Rejection {
    let mut status = Status::new(&StatusCode::UNAUTHORIZED);
    status.headers_mut().insert(
        http::header::WWW_AUTHENTICATE,
        HeaderValue::from_static("Basic"),
    );
    status.into()
}

/// Parse the Authorization header for user credentials
fn credentials_from_header() -> BoxedFilter<(String, String)> {
    let auth_header: &'static str = http::header::AUTHORIZATION.as_str();
    warp::filters::header::header::<String>(auth_header)
        .and_then(move |val: String| async move {
            let params = val.split_whitespace().collect::<Vec<&str>>();

            let method = params
                .get(0)
                .ok_or_else(|| status::invalid_header_error(auth_header))?;

            if !method.eq_ignore_ascii_case("basic") {
                return Err(status::invalid_header_error(auth_header));
            }

            let encoded = params
                .get(1)
                .ok_or_else(|| status::invalid_header_error(auth_header))?;

            let decoded =
                base64::decode(encoded).map_err(|_| status::invalid_header_error(auth_header))?;

            let decoded = std::str::from_utf8(decoded.as_slice())
                .map_err(|_| status::invalid_header_error(auth_header))?;

            let colon = decoded
                .find(':')
                .ok_or_else(|| status::invalid_header_error(auth_header))?;
            let (username, password) = decoded.split_at(colon);
            Ok((username.to_string(), password[1..].to_string()))
        })
        .untuple_one()
        .boxed()
}

/// Given the user credentials and a database connection, authenticate and,
/// if authenticated, create a User struct and return it.
async fn user_from_credentials(
    user: String,
    pass: String,
    conn: db::Connection,
) -> Result<User, Rejection> {
    let (user, auth) = DBUser::db_from_username(user, &conn)?.into();

    if auth.is_valid(&pass)? {
        Ok(user)
    } else {
        Err(reject_login_required())
    }
}

/// A Filter that provides an instance of the authenticated user.
pub fn user_filter() -> BoxedFilter<(User,)> {
    credentials_from_header()
        .and(db::conn_filter())
        .and_then(user_from_credentials)
        .boxed()
}

/// An endpoint that tests if the user credentials are correct and nothing more.
pub fn login_filter() -> BoxedFilter<(Status<Empty>,)> {
    user_filter()
        .map(|_| Status::new(&StatusCode::OK))
        .boxed()
}
