use crate::config;
use crate::status;
use warp::filters::BoxedFilter;
use warp::{Filter, Rejection};

pub use tavern_db::{Connection, Error, TryFromRow, TryFromUuid, DBInsertSingle, DBUpdateSingle, DBDeleteSingle};

embed_migrations!();

pub async fn init() -> Result<(), Error> {
    let mut conn = tavern_db::get_connection().await?;
    embedded_migrations::run(&conn)
        .map_err(Error::Migration)
}

async fn get_filter_connection() -> Result<Connection, Rejection> {
    tavern_db::get_connection()
        .await
        .map_err(|err| status::server_error_into_rejection(err.to_string()))
}

pub fn conn_filter() -> BoxedFilter<(Connection,)> {
    warp::any().and_then(get_filter_connection).boxed()
}
