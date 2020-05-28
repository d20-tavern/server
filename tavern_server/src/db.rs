use crate::config;
use crate::status;
use warp::filters::BoxedFilter;
use warp::{Filter, Rejection};

pub use tavern_db::{Connection, Error, TryFromRow, TryFromUuid, DBInsertSingle, DBUpdateSingle, DBDeleteSingle};

async fn get_filter_connection() -> Result<Connection, Rejection> {
    tavern_db::get_connection()
        .await
        .map_err(|err| status::server_error_into_rejection(err.to_string()))
}

pub fn conn_filter() -> BoxedFilter<(Connection,)> {
    warp::any().and_then(get_filter_connection).boxed()
}
