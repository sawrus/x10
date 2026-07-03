use std::{env, net::IpAddr, path::PathBuf};

pub struct Config {
    pub host: IpAddr,
    pub port: u16,
    pub database_path: PathBuf,
    pub uploads_path: PathBuf,
    pub web_dist_path: PathBuf,
    pub admin_username: String,
    pub admin_password_hash: String,
    pub admin_session_secret: String,
    pub admin_session_secure: bool,
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
        let admin_username =
            env::var("X10_ADMIN_USERNAME").expect("X10_ADMIN_USERNAME must be set");
        let admin_password_hash =
            env::var("X10_ADMIN_PASSWORD_HASH").expect("X10_ADMIN_PASSWORD_HASH must be set");
        let admin_session_secret =
            env::var("X10_ADMIN_SESSION_SECRET").expect("X10_ADMIN_SESSION_SECRET must be set");
        let admin_session_secure = env::var("X10_ADMIN_SESSION_SECURE")
            .ok()
            .map(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "on"))
            .unwrap_or(false);

        Self {
            host,
            port,
            database_path,
            uploads_path,
            web_dist_path,
            admin_username,
            admin_password_hash,
            admin_session_secret,
            admin_session_secure,
        }
    }
}
