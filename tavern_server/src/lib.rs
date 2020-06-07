// This macro_use is necessary until diesel 2.0
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use warp::filters::BoxedFilter;
use warp::{Filter, Reply, Rejection};

pub mod api;
pub mod auth;
pub mod config;
pub mod db;
pub mod forms;
pub mod pathfinder;
mod schema;
pub mod status;

use crate::api::{GetById, GetAll, Insert, Update, DeleteById, APIPath, Filters};
use crate::pathfinder::character::{Character, Race, RaceType, RaceSubtype};
use uuid::Uuid;
use crate::status::Error;
use nebula_status::{Status, StatusCode, Empty, Error as NebulaError};
use std::convert::Infallible;

async fn recover_rejection(rej: Rejection) -> Result<impl Reply, Infallible> {
    let reply: Box<dyn Reply> = match Status::<Error>::recover(rej) {
        Ok(status) => Box::new(status),
        Err(rej) => {
            match rej {
                NebulaError::NotStatus(rej) => {
                    match Status::<Empty>::recover(rej) {
                        Ok(status) => Box::new(status),
                        Err(_) => Box::new(Status::new(&StatusCode::INTERNAL_SERVER_ERROR)),
                    }
                }
            }
        },
    };

    Ok(reply)
}

/// Generate a warp Filter containing the full server and return it.
pub fn setup_server() -> BoxedFilter<(impl Reply,)> {
    let login = warp::get()
        .and(warp::path("login"))
        .and(auth::login_filter());
    let register = warp::post()
        .and(warp::path("register"))
        .and(auth::register_filter());

    login
        .or(register)
        .or(RaceSubtype::filters(None))
        //.or(Character::filters(None))
        .recover(recover_rejection)
        .boxed()
}
