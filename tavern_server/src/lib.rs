use warp::{Filter, Reply};
use warp::filters::BoxedFilter;

pub mod auth;
pub mod config;
pub mod db;
pub mod forms;
pub mod status;

pub fn setup_server() -> BoxedFilter<(impl Reply,)> {
    let login = warp::get()
                    .and(warp::path("login"))
                    .and(auth::login_filter());
    let register = warp::post()
                    .and(warp::path("register"))
                    .and(auth::register_filter());

    warp::any().and(
        login
        .or(register)
    ).boxed()
}
