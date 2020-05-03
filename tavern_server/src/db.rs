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
    #[test]
    fn postgres_database_init() {
        assert!(true);
    }
}

#[derive(StructOpt, Debug)]
pub struct PostgreSQLOpt {
    #[structopt(long = "db-host", env = "TAVERN_DB_HOST", help = "the domain name or IP address of the database host")]
    host: String,
    #[structopt(long = "db-port", env = "TAVERN_DB_PORT", default_value = "5432", help = "the port PostgreSQL is listening to on the host")]
    port: u16,
    #[structopt(long = "db-name", env = "TAVERN_DB_NAME", help = "the name of the database Tavern will use")]
    database: String,
    #[structopt(long = "db-user", env = "TAVERN_DB_USER", help = "the username for the database")]
    user: String,
    #[structopt(long = "db-pass", env = "TAVERN_DB_PASS", help = "the password for the database user")]
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
