use tavern_server::db::PostgreSQLOpt;
use structopt::StructOpt;


#[derive(StructOpt, Debug)]
struct Config {
    #[structopt(flatten)]
    database: PostgreSQLOpt,
}

fn main() {
    let _config = Config::from_args();
}
