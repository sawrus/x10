use std::{env, net::IpAddr, path::PathBuf};

pub struct Config {
    pub host: IpAddr,
    pub port: u16,
    pub database_path: PathBuf,
    pub uploads_path: PathBuf,
    pub web_dist_path: PathBuf,
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
        let uploads_path = env::var("X10_UPLOADS_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("data/uploads"));
        let web_dist_path = env::var("X10_WEB_DIST_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("web/dist"));

        Self {
            host,
            port,
            database_path,
            uploads_path,
            web_dist_path,
        }
    }
}
