use std::{net::SocketAddr, sync::Arc};

use metrics_exporter_prometheus::PrometheusBuilder;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
use x10_backend::{
    api::{AppState, build_router},
    application::ProgressionService,
    config::Config,
    infrastructure::SqliteRepository,
};

#[tokio::main]
async fn main() {
    init_tracing();

    let config = Config::from_env();
    let metrics_handle = PrometheusBuilder::new()
        .install_recorder()
        .expect("prometheus recorder should install");

    let repository = Arc::new(
        SqliteRepository::new(&config.database_path).expect("sqlite repository should initialize"),
    );
    let service = Arc::new(ProgressionService::new(
        repository,
        config.uploads_path.clone(),
    ));
    let state = AppState::new(
        service,
        Some(metrics_handle),
        config.web_dist_path.clone(),
        config.admin_username.clone(),
        config.admin_password_hash.clone(),
        config.admin_session_secret.clone(),
        config.admin_session_secure,
    );
    let app = build_router(state);

    let address = SocketAddr::from((config.host, config.port));
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("listener should bind");

    tracing::info!(%address, "x10 backend listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server should run");
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(fmt::layer().json())
        .init();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("ctrl-c handler should install");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("signal handler should install")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
