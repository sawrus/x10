use std::{env, net::IpAddr, path::PathBuf};

pub struct Config {
    pub host: IpAddr,
    pub port: u16,
    pub database_path: PathBuf,
}

impl Config {
    pub fn from_env() -> Self {
        let host = env::var("X10_HOST")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(IpAddr::from([127, 0, 0, 1]));

        let port = env::var("X10_PORT")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(3000);

        let database_path = env::var("X10_DATABASE_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("data/x10.sqlite3"));

        Self {
            host,
            port,
            database_path,
        }
    }
}
