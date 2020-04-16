use std::fmt;
use structopt::StructOpt;

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

        assert!(conn_string.contains("host=host.example.com"));
        assert!(conn_string.contains("port=5432"));
        assert!(conn_string.contains("dbname=test"));
        assert!(conn_string.contains("user=foo"));
        assert!(conn_string.contains("password=bar"));
    }
}

#[derive(StructOpt, Debug)]
pub struct PostgreSQLOpt {
    #[structopt(long = "db-host", env = "TAVERN_DB_HOST")]
    host: String,
    #[structopt(long = "db-port", env = "TAVERN_DB_PORT", default_value = "5432")]
    port: u16,
    #[structopt(long = "db-name", env = "TAVERN_DB_NAME")]
    database: String,
    #[structopt(long = "db-user", env = "TAVERN_DB_USER")]
    user: String,
    #[structopt(long = "db-pass", env = "TAVERN_DB_PASS")]
    pass: String,
}

impl fmt::Display for PostgreSQLOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "host={} port={} dbname={} user={} password={}",
            self.host, self.port, self.database, self.user, self.pass
        )
    }
}
