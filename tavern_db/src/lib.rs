use async_trait::async_trait;
use futures::executor::block_on;
use sqlx::pool::PoolConnection;
use uuid::Uuid;
use lazy_static::lazy_static;
use sqlx::{Executor, PgConnection, PgPool, postgres::PgRow};
use std::convert::{TryFrom, TryInto};
use std::fmt::{self, Display};
use std::sync::Arc;
use structopt::StructOpt;
use tokio::sync::RwLock;
pub use tavern_derive::*;

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

pub async fn init() -> Result<(), Error> {
    let mut conn: Connection = get_connection().await?;
    let sql = std::str::from_utf8(include_bytes!("db_tables.sql"))
        .map_err(Error::LoadQuery)?;
    conn.execute(sql)
        .await
        .map_err(Error::RunQuery)?;
    Ok(())
}


lazy_static! {
    static ref POOL: Arc<RwLock<PgPool>> = {
        let pool = PostgreSQLOpt::from_args().try_into().unwrap();
        let lock = RwLock::new(pool);
        Arc::new(lock)
    };
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
}

impl TryFrom<PostgreSQLOpt> for PgPool {
    type Error = sqlx::Error;
    fn try_from(opt: PostgreSQLOpt) -> Result<Self, Self::Error> {
        block_on(PgPool::new(opt.to_string().as_ref()))
    }
}

impl fmt::Display for PostgreSQLOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
//            "host={} port={} dbname={} user={} password={}",
//            self.host, self.port, self.database, self.user, self.pass
            "postgresql://{}:{}@{}:{}/{}",
            self.user, self.pass, self.host, self.port, self.database
            )
    }
}

pub async fn get_connection() -> Result<Connection, Error> {
    (*POOL)
        .read()
        .await
        .acquire()
        .await
        .map_err(Error::Connection)
}

pub type Connection = PoolConnection<PgConnection>;

#[derive(Debug)]
pub enum Error {
    Connection(sqlx::Error),
    Transaction(sqlx::Error),
    LoadQuery(std::str::Utf8Error),
    RunQuery(sqlx::Error),
    NoRows,
    UserUnauthorized(Uuid),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Connection(err) => write!(f, "{}", err.to_string()),
            Error::Transaction(err) => write!(f, "{}", err.to_string()),
            Error::LoadQuery(err) => write!(f, "{}", err.to_string()),
            Error::RunQuery(err) => write!(f, "{}", err.to_string()),
            Error::NoRows => write!(f, "No rows"),
            Error::UserUnauthorized(id) => write!(f, "User {} is unauthorized", id),
        }
    }
}

#[async_trait]
pub trait TryFromUuid {
    async fn try_from_uuid(id: Uuid, user: &Uuid) -> Result<Self, Error> where Self: Sized;
}

#[async_trait]
pub trait TryFromRow {
    async fn try_from_row(row: &PgRow<'_>, user: &Uuid) -> Result<Self, Error> where Self: Sized + 'static;
}

#[async_trait]
pub trait DBInsertSingle {
    async fn db_insert(&self, conn: &Connection) -> Result<(), Error>;
}

#[async_trait]
pub trait DBUpdateSingle {
    async fn db_update(&self, conn: &Connection) -> Result<(), Error>;
}

#[async_trait]
pub trait DBDeleteSingle {
    async fn db_delete(&self, conn: &Connection) -> Result<(), Error>;
}

// Error codes come from https://www.postgresql.org/docs/10/errcodes-appendix.html
pub const PG_ERROR_CHECK_VIOLATION: &str = "23514";
pub const PG_ERROR_FOREIGN_KEY_VIOLATION: &str = "23503";
pub const PG_ERROR_NOT_NULL_VIOLATION: &str = "23502";
pub const PG_ERROR_RESTRICT_VIOLATION: &str = "23001";
pub const PG_ERROR_UNIQUE_VIOLATION: &str = "23505";
