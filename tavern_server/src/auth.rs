use crate::forms;
use crate::status::{Error, Success};
use crate::{config, db, status};
use argon2::{self, Config, ThreadMode, Variant, Version};
use bytes::Bytes;
use nebula_form::Form;
use nebula_status::{Status, StatusCode};
use rand::RngCore;
use serde::Serialize;
use sqlx::types::Uuid;
use sqlx::Error as SQLError;
use sqlx::{Connection, PgConnection};
use std::convert::TryFrom;
use structopt::StructOpt;
use warp::filters::BoxedFilter;
use warp::reject::Rejection;
use warp::Filter;
use futures::executor::block_on;

pub const ARGON2_HASH_LENGTH: u32 = 32;
pub const ARGON2_SALT_LENGTH: usize = 32;

#[cfg(test)]
mod tests {
    use super::*;
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

        let (user, auth) = registration_to_user_auth(info, salt.clone(), conf.clone()).await.unwrap();

        assert_eq!(exp_user, user);
        assert_eq!(expected_hash, auth.hash);
        assert_eq!(salt, auth.salt.as_slice());
        assert_eq!(conf, auth.config);
    }
}

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
        config.variant = Variant::Argon2i;
        config.version = Version::Version13;
        config.thread_mode = ThreadMode::Parallel;
        config.mem_cost = opt.memory;
        config.lanes = opt.threads;
        config.time_cost = opt.time_cost;
        config
    }
}

const FIELD_USER_ID: &'static str = "user-id";
const FIELD_EMAIL: &'static str = "email";
const FIELD_IS_ADMIN: &'static str = "is-admin";
const FIELD_PASSWORD: &'static str = "password";
const FIELD_USERNAME: &'static str = "username";

#[derive(Serialize, Clone, Debug)]
pub(crate) struct User {
    pub(crate) id: Option<Uuid>,
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) is_admin: bool,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        if self.id.is_some() && other.id.is_some() {
            self.id.unwrap() == other.id.unwrap()
        } else {
            self.username == other.username ||
            self.email    == other.email
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
        let mut id = form
            .get(FIELD_USER_ID)
            .map(|field| {
                field
                    .as_text()
                    .map(|val| {
                        Uuid::parse_str(val)
                            .map_err(|err| {
                                forms::field_is_invalid_error(FIELD_USER_ID)
                            })
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

struct UserAuth {
    hash: Vec<u8>,
    salt: Vec<u8>,
    config: argon2::Config<'static>,
}

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

fn generate_salt() -> BoxedFilter<(Vec<u8>,)> {
    warp::any()
        .and_then(|| async move {
            let mut salt: Vec<u8> = Vec::with_capacity(ARGON2_SALT_LENGTH);
            rand::thread_rng()
                .try_fill_bytes(&mut salt)
                .map(|_| salt)
                .map_err(|err| {
                    Status::with_message(&StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err))
                })
                .map_err(|err| -> Rejection { err.into() })
        })
        .boxed()
}

fn get_registration_info() -> BoxedFilter<(RegistrationInfo,)> {
    warp::filters::method::post()
        .and(nebula_form::form_filter())
        .and_then(|form: Form| async move { 
            RegistrationInfo::try_from(form)
                .map(|mut info| {
                    info.user.id = None;
                    info
                })
        })
        .boxed()
}

fn get_argon2_config() -> BoxedFilter<(argon2::Config<'static>,)> {
    config::filter()
        .map(|conf: &'static config::Config| conf.argon2.clone().into())
        .boxed()
}

async fn hash_password(
    password: &[u8],
    salt: &[u8],
    config: &argon2::Config<'static>,
) -> Result<Vec<u8>, Rejection> {
    argon2::hash_raw(password, salt, config)
        .map_err(|err| status::server_error_into_rejection(err.to_string()).into())
}

async fn register_in_database(
    user: User,
    auth: UserAuth,
    conn: db::Connection,
) -> Result<Status<Success<User>>, Rejection> {
    let mut tx = conn
        .begin()
        .await
        .map_err(|err| status::server_error_into_rejection(err.to_string()))?;

    let id = Uuid::new_v4();

    let query = sqlx::query(
        r"INSERT INTO Users
    (id, email, username, pass_hash, salt, time_cost, memory, threads)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
    ON CONFLICT DO NOTHING",
    )
    .bind(&id)
    .bind(&user.email)
    .bind(&user.username)
    .bind(&auth.hash)
    .bind(&auth.salt)
    .bind(&auth.config.time_cost)
    .bind(&auth.config.mem_cost)
    .bind(&auth.config.lanes);

    query.execute(&mut tx).await.map_err(|err| {
        match err {
            SQLError::Database(dberr) => {
                match dberr.code() {
                    Some(db::PG_ERROR_UNIQUE_VIOLATION) => {
                        match dberr.column_name() {
                            None => status::server_error_into_rejection(dberr.to_string()),
                            Some(name) => match name {
                                // Before updating this code with new columns, ensure that no
                                // sensitive information will end up in the error message.
                                "email" | "username" => Status::with_data(
                                    &StatusCode::BAD_REQUEST,
                                    Error::new(format!("user with that {} already exists", name)),
                                )
                                .into(),
                                _ => status::server_error_into_rejection(dberr.to_string()),
                            },
                        }
                    }
                    _ => status::server_error_into_rejection(dberr.to_string()),
                }
            }
            _ => status::server_error_into_rejection(err.to_string()),
        }
    })?;

    tx.commit()
        .await
        .map(|_| Status::with_data(&StatusCode::OK, Success::new(user)))
        .map_err(|err| status::server_error_into_rejection(err.to_string()))
}

async fn registration_to_user_auth(
    info: RegistrationInfo,
    salt: Vec<u8>,
    config: argon2::Config<'static>,
) -> Result<(User, UserAuth), Rejection> {
    let hash = hash_password(&info.password.as_bytes(), salt.as_slice(), &config).await?;
    let user_auth = UserAuth { hash, salt, config };
    Ok((info.user, user_auth))
}

pub(crate) fn register_filter() -> BoxedFilter<(Status<Success<User>>,)> {
    get_registration_info()
        .and(generate_salt())
        .and(get_argon2_config())
        .and_then(registration_to_user_auth)
        .untuple_one()
        .and(db::conn_filter())
        .and_then(register_in_database)
        .boxed()
}
