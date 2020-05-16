use crate::config;
use crate::status;
use futures::executor::block_on;
use lazy_static::lazy_static;
use sqlx::pool::PoolConnection;
use sqlx::{Connection as _, PgConnection, PgPool};
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::sync::Arc;
use structopt::StructOpt;
use tokio::sync::RwLock;
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
        };

        let conn_string = ps_opt.to_string();

        assert_eq!(conn_string.as_str(), "postgresql://foo:bar@host.example.com:5432/test");
        //assert!(conn_string.contains("host=host.example.com"));
        //assert!(conn_string.contains("port=5432"));
        //assert!(conn_string.contains("dbname=test"));
        //assert!(conn_string.contains("user=foo"));
        //assert!(conn_string.contains("password=bar"));
    }
    #[test]
    fn postgres_database_init() {
        assert!(true);
    }
}

#[derive(Debug)]
pub enum Error {
    Connection(sqlx::Error),
    Transaction(sqlx::Error),
    LoadQuery(std::str::Utf8Error),
    RunQuery(sqlx::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Connection(err) => write!(f, "{}", err.to_string()),
            Error::Transaction(err) => write!(f, "{}", err.to_string()),
            Error::LoadQuery(err) => write!(f, "{}", err.to_string()),
            Error::RunQuery(err) => write!(f, "{}", err.to_string()),
        }
    }
}

pub async fn init() -> Result<(), Error> {
    let conn: Connection = get_connection().await?;
    let mut tx = conn.begin().await
        .map_err(|err| Error::Transaction(err))?;
    let sql = std::str::from_utf8(include_bytes!("db_tables.sql"))
        .map_err(|err| Error::LoadQuery(err))?;

    for sqline in sql.split("\n\n") {
        println!("{}", sqline);
        sqlx::query(sqline).execute(&mut tx).await
            .map_err(|err| Error::RunQuery(err))?;
    }

    tx.commit().await
        .map_err(|err| Error::Transaction(err))?;
    Ok(())
}

// Error codes come from https://www.postgresql.org/docs/10/errcodes-appendix.html
pub const PG_ERROR_CHECK_VIOLATION: &'static str = "23514";
pub const PG_ERROR_FOREIGN_KEY_VIOLATION: &'static str = "23503";
pub const PG_ERROR_NOT_NULL_VIOLATION: &'static str = "23502";
pub const PG_ERROR_RESTRICT_VIOLATION: &'static str = "23001";
pub const PG_ERROR_UNIQUE_VIOLATION: &'static str = "23505";

pub type Connection = PoolConnection<PgConnection>;

lazy_static! {
    static ref POOL: Arc<RwLock<PgPool>> = {
        let pool = config::config().database.clone().try_into().unwrap();
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
    (*POOL).read().await.acquire().await
        .map_err(|err| Error::Connection(err))
}

async fn get_filter_connection() -> Result<Connection, Rejection> {
    get_connection()
        .await
        .map_err(|err| status::server_error_into_rejection(err.to_string()))
}

pub fn conn_filter() -> BoxedFilter<(Connection,)> {
    warp::any().and_then(get_filter_connection).boxed()
}
