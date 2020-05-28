use crate::auth::Argon2Opt;
use tavern_db::PostgreSQLOpt;
use lazy_static::lazy_static;
use structopt::StructOpt;
use warp::filters::BoxedFilter;
use warp::Filter;

// Creates a private single instance of the configuration.
// Filters can then use the filter() method to get an immutable reference
// to this static variable.
// No Arc<> is used here because threads only receive immutable references.
lazy_static! {
    static ref CONFIG: Config = Config::from_args();
}

#[derive(StructOpt, Debug)]
pub(crate) struct Config {
    #[structopt(flatten)]
    pub(crate) database: PostgreSQLOpt,
    #[structopt(flatten)]
    pub(crate) argon2: Argon2Opt,
}

/// A getter returning an immutable reference to the server configuration.
/// Should only be used by functions outside of warp Filters. For Filters,
/// use filter() instead.
pub(crate) fn config() -> &'static Config {
    &*CONFIG
}

/// A Filter providing access to a single, global, immutable configuration
/// object initialized using lazy_static and structopt. Other filters can
/// use this to read server configuration without directly accessing global
/// variables.
pub(crate) fn filter() -> BoxedFilter<(&'static Config,)> {
    warp::any().map(|| &*CONFIG).boxed()
}
