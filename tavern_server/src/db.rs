use crate::status;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::result;
use lazy_static::lazy_static;
use std::fmt::{self, Display};
use std::sync::Arc;
use structopt::StructOpt;
pub use tavern_derive::*;
use tokio::sync::RwLock;
use uuid::Uuid;
use warp::filters::BoxedFilter;
use warp::{Filter, Rejection};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn postgres_opt_to_connstr() {
        let ps_opt = PostgreSQLOpt {
            host: String::from("host.example.com"),
            port: 5432u16,
            database: String::from("test"),
            user: String::from("foo"),
            pass: String::from("bar"),
            max_connections: 5,
        };

        let conn_string = ps_opt.to_string();

        assert_eq!(
            conn_string.as_str(),
            "postgresql://foo:bar@host.example.com:5432/test"
        );
    }
    #[test]
    fn postgres_database_init() {
        assert!(true);
    }
}

lazy_static! {
    static ref POOL: Arc<RwLock<Pool<TavernConnectionManager>>> = {
        let pool = PostgreSQLOpt::from_args().into();
        let lock = RwLock::new(pool);
        Arc::new(lock)
    };
}

embed_migrations!();

pub async fn init() -> Result<(), Error> {
    let mut conn = get_connection().await?;
    embedded_migrations::run(&conn).map_err(Error::Migration)
}

async fn get_filter_connection() -> Result<Connection, Rejection> {
    get_connection()
        .await
        .map_err(|err| status::server_error_into_rejection(err.to_string()))
}

pub fn conn_filter() -> BoxedFilter<(Connection,)> {
    warp::any().and_then(get_filter_connection).boxed()
}

#[derive(StructOpt, Clone, Debug)]
pub struct PostgreSQLOpt {
    #[structopt(
        long = "db-host",
        env = "TAVERN_DB_HOST",
        help = "the domain name or IP address of the database host"
    )]
    host: String,
    #[structopt(
        long = "db-port",
        env = "TAVERN_DB_PORT",
        default_value = "5432",
        help = "the port PostgreSQL is listening to on the host"
    )]
    port: u16,
    #[structopt(
        long = "db-name",
        env = "TAVERN_DB_NAME",
        help = "the name of the database Tavern will use"
    )]
    database: String,
    #[structopt(
        long = "db-user",
        env = "TAVERN_DB_USER",
        help = "the username for the database"
    )]
    user: String,
    #[structopt(
        long = "db-pass",
        env = "TAVERN_DB_PASS",
        help = "the password for the database user"
    )]
    pass: String,
    #[structopt(
        long = "db-max-conn",
        env = "TAVERN_DB_MAX_CONNECTIONS",
        help = "the maximum number of database connections",
        default_value = "10"
    )]
    max_connections: u32,
}

pub type TavernConnectionManager = ConnectionManager<PgConnection>;
pub type Connection = PooledConnection<TavernConnectionManager>;

impl From<PostgreSQLOpt> for Pool<TavernConnectionManager> {
    fn from(opt: PostgreSQLOpt) -> Self {
        let manager = TavernConnectionManager::new(opt.to_string().as_str());
        Pool::builder()
            .max_size(opt.max_connections)
            .build(manager)
            .unwrap()
    }
}

impl fmt::Display for PostgreSQLOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "postgresql://{}:{}@{}:{}/{}",
            self.user, self.pass, self.host, self.port, self.database
        )
    }
}

pub async fn get_connection() -> Result<Connection, Error> {
    (*POOL).read().await.get().map_err(Error::Connection)
}

#[derive(Debug)]
pub enum Error {
    Connection(::r2d2::Error),
    Migration(diesel_migrations::RunMigrationsError),
    RunQuery(result::Error),
    NoRows,
    UserUnauthorized(Uuid),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Connection(err) => write!(f, "{}", err.to_string()),
            Error::Migration(err) => write!(f, "{}", err.to_string()),
            Error::RunQuery(err) => write!(f, "{}", err.to_string()),
            Error::NoRows => write!(f, "No rows"),
            Error::UserUnauthorized(id) => write!(f, "User {} is unauthorized", id),
        }
    }
}

pub trait GetById {
    fn db_get_by_id(id: &Uuid, conn: &crate::db::Connection) -> Result<Self, Error>
    where
        Self: Sized;
}

pub trait GetAll {
    fn db_get_all(conn: &crate::db::Connection) -> Result<Vec<Self>, Error>
    where
        Self: Sized;
}

pub trait Insert {
    fn db_insert(&self, conn: &crate::db::Connection) -> Result<(), Error>;
}

pub trait Update {
    fn db_update(&self, conn: &crate::db::Connection) -> Result<(), Error>;
}

pub trait Delete {
    fn db_delete(&self, conn: &crate::db::Connection) -> Result<(), Error>;
}

// Error codes come from https://www.postgresql.org/docs/10/errcodes-appendix.html
pub const PG_ERROR_CHECK_VIOLATION: &str = "23514";
pub const PG_ERROR_FOREIGN_KEY_VIOLATION: &str = "23503";
pub const PG_ERROR_NOT_NULL_VIOLATION: &str = "23502";
pub const PG_ERROR_RESTRICT_VIOLATION: &str = "23001";
pub const PG_ERROR_UNIQUE_VIOLATION: &str = "23505";
