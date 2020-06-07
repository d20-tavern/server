use crate::status::{self, Error as StatusError};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::result::Error as DieselError;
use lazy_static::lazy_static;
use std::fmt::{self, Display};
use std::sync::Arc;
use structopt::StructOpt;
use tokio::sync::RwLock;
use uuid::Uuid;
use warp::filters::BoxedFilter;
use warp::{Filter, Rejection};

pub use tavern_derive::*;
use nebula_status::{Status, StatusCode};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn postgres_opt_to_connstr() {
        let ps_opt = PostgreSQLOpt {
            host: String::from("host.example.com"),
            db_port: 5432u16,
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

pub async fn init() -> Result<(), String> {
    let conn = get_connection().await
        .map_err(|err| format!("error while getting connection: {:#?}", err))?;
    embedded_migrations::run(&conn).map_err(|err| format!("error while running migrations: {:#?}", err))
}

pub async fn revert() -> Result<(), String> {
    let conn = get_connection().await
        .map_err(|err| format!("error while getting connection: {:#?}", err))?;
    diesel_migrations::revert_latest_migration(&conn).map_err(|err| format!("error while reverting migrations: {:#?}", err))?;
    diesel_migrations::revert_latest_migration(&conn).map_err(|err| format!("error while reverting migrations: {:#?}", err))?;
    diesel_migrations::revert_latest_migration(&conn).map_err(|err| format!("error while reverting migrations: {:#?}", err))
        .map(|_| ())
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
    db_port: u16,
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
            self.user, self.pass, self.host, self.db_port, self.database
        )
    }
}

pub async fn get_connection() -> Result<Connection, Error> {
    (*POOL).read().await.get().map_err(Error::Connection)
}

#[derive(Debug)]
pub enum Error {
    Connection(::r2d2::Error),
    InvalidValues(Vec<String>),
    Migration(diesel_migrations::RunMigrationsError),
    RunQuery(DieselError),
    NoRows,
    UserUnauthorized(Uuid),
    Other(String),
}

impl From<DieselError> for Error {
    fn from(err: DieselError) -> Self {
        Error::RunQuery(err)
    }
}

impl From<Error> for Rejection {
    fn from(err: Error) -> Self {
        match err {
            Error::InvalidValues(list) => {
                let error = StatusError::new(format!("invalid values for {}", list.join(", ")));
                Status::with_data(&StatusCode::BAD_REQUEST, error).into()
            },
            Error::UserUnauthorized(id) => {
                Status::new(&StatusCode::UNAUTHORIZED).into()
            },
            err => {
                status::server_error_into_rejection(err.to_string())
            }
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Connection(err) => write!(f, "{}", err.to_string()),
            Error::InvalidValues(list) => write!(f, "{}", list.join(", ")),
            Error::Migration(err) => write!(f, "{}", err.to_string()),
            Error::RunQuery(err) => write!(f, "{}", err.to_string()),
            Error::NoRows => write!(f, "No rows"),
            Error::UserUnauthorized(id) => write!(f, "User {} is unauthorized", id),
            Error::Other(err) => write!(f, "{}", err),
        }
    }
}

pub trait TryFromDb {
    type DBType;
    fn try_from_db(other: Self::DBType, conn: &Connection) -> Result<Self, Error> where Self: Sized;
}

pub trait IntoDbWithId {
    type DBType;
    fn into_db(self, id: Uuid) -> Self::DBType;
}

pub trait IntoDb {
    type DBType;
    fn into_db(self) -> Self::DBType;
}

/// This trait represents a type that is based on a single DB table. It may have foreign
/// keys, but there are e.g. no junction tables that need to be updated when this type is
/// updated.
pub trait StandaloneDbMarker {}

pub trait GetById {
    fn db_get_by_id(id: &Uuid, conn: &Connection) -> Result<Self, Error>
    where
        Self: Sized;
}

impl<T, U> GetById for T where T: TryFromDb<DBType=U>, U: GetById {
    fn db_get_by_id(id: &Uuid, conn: &Connection) -> Result<Self, Error> where
        Self: Sized {
        T::try_from_db(U::db_get_by_id(id, conn)?, conn)
    }
}

pub trait GetAll {
    fn db_get_all(conn: &Connection) -> Result<Vec<Self>, Error>
    where
        Self: Sized;
}

impl<T, U> GetAll for T where T: TryFromDb<DBType=U>, U: GetAll {
    fn db_get_all(conn: &Connection) -> Result<Vec<Self>, Error> where
        Self: Sized {
        U::db_get_all(conn)?
            .into_iter()
            .map(|item| T::try_from_db(item, conn))
            .collect()
    }
}

pub trait GetAllUnderParent {
    fn db_get_all_under(parent_id: &Uuid, conn: &Connection) -> Result<Vec<Self>, Error> where Self: Sized;
}

impl<T, U> GetAllUnderParent for T where T: TryFromDb<DBType=U>, U: GetAllUnderParent {
    fn db_get_all_under(id: &Uuid, conn: &Connection) -> Result<Vec<Self>, Error> where
        Self: Sized {
        U::db_get_all_under(id, conn)?
            .into_iter()
            .map(|item| T::try_from_db(item, conn))
            .collect()
    }
}

pub trait Insert {
    fn db_insert(&self, conn: &Connection) -> Result<(), Error>;
}

impl<T,U> Insert for (T, Uuid) where T: IntoDbWithId<DBType=U> + Clone + StandaloneDbMarker, U: Insert {
    fn db_insert(&self, conn: &Connection) -> Result<(), Error> {
        let (item, id) = self;
        let db_item: U = T::clone(item)
            .into_db(Uuid::clone(id));
        db_item.db_insert(conn)
    }
}

impl<T,U> Insert for T where T: IntoDb<DBType=U> + Clone + StandaloneDbMarker, U: Insert {
    fn db_insert(&self, conn: &Connection) -> Result<(), Error> {
        let db_item: U = T::clone(self)
            .into_db();
        db_item.db_insert(conn)
    }
}

pub trait Update {
    fn db_update(&self, conn: &Connection) -> Result<(), Error>;
}

impl<T,U> Update for (T, Uuid) where T: IntoDbWithId<DBType=U> + Clone + StandaloneDbMarker, U: Update {
    fn db_update(&self, conn: &Connection) -> Result<(), Error> {
        let (item, id) = self;
        let db_item: U = T::clone(item)
            .into_db(Uuid::clone(id));
        db_item.db_update(conn)
    }
}

impl<T,U> Update for T where T: IntoDb<DBType=U> + Clone + StandaloneDbMarker, U: Update {
    fn db_update(&self, conn: &Connection) -> Result<(), Error> {
        let db_item: U = T::clone(self)
            .into_db();
        db_item.db_update(conn)
    }
}

pub trait Delete {
    fn db_delete(&self, conn: &Connection) -> Result<(), Error>;
}

impl<T,U> Delete for (T, Uuid) where T: IntoDbWithId<DBType=U> + Clone + StandaloneDbMarker, U: Delete {
    fn db_delete(&self, conn: &Connection) -> Result<(), Error> {
        let (item, id) = self;
        let db_item: U = T::clone(item)
            .into_db(Uuid::clone(id));
        db_item.db_delete(conn)
    }
}

impl<T,U> Delete for T where T: IntoDb<DBType=U> + Clone + StandaloneDbMarker, U: Delete {
    fn db_delete(&self, conn: &Connection) -> Result<(), Error> {
        let db_item: U = T::clone(self)
            .into_db();
        db_item.db_delete(conn)
    }
}

pub trait DeleteById {
    fn db_delete_by_id(id: &Uuid, conn: &Connection) -> Result<(), Error>;
}

impl<T,U> DeleteById for T where T: TryFromDb<DBType=U> + StandaloneDbMarker , U: DeleteById {
    fn db_delete_by_id(id: &Uuid, conn: &Connection) -> Result<(), Error> {
        U::db_delete_by_id(id, conn)
    }
}

// Error codes come from https://www.postgresql.org/docs/10/errcodes-appendix.html
pub const PG_ERROR_CHECK_VIOLATION: &str = "23514";
pub const PG_ERROR_FOREIGN_KEY_VIOLATION: &str = "23503";
pub const PG_ERROR_NOT_NULL_VIOLATION: &str = "23502";
pub const PG_ERROR_RESTRICT_VIOLATION: &str = "23001";
pub const PG_ERROR_UNIQUE_VIOLATION: &str = "23505";
