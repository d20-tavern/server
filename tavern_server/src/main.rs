use tavern_server::auth::Argon2Opt;
use tavern_server::db::PostgreSQLOpt;
use structopt::StructOpt;


#[derive(StructOpt, Debug)]
struct Config {
    #[structopt(flatten)]
    database: PostgreSQLOpt,
    #[structopt(flatten)]
    argon2: Argon2Opt,
}

fn main() {
    let _config = Config::from_args();
}
