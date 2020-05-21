use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

pub mod auth;
pub mod config;
pub mod db;
pub mod forms;
pub mod status;

/// Generate a warp Filter containing the full server and return it.
pub fn setup_server() -> BoxedFilter<(impl Reply,)> {
    let login = warp::get()
        .and(warp::path("login"))
        .and(auth::login_filter());
    let register = warp::post()
        .and(warp::path("register"))
        .and(auth::register_filter());

    warp::any().and(login.or(register)).boxed()
}
