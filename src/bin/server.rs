use axum::{routing::post, Router};
use clap::Parser;
use rust_openapi_cicd::{
    app::{app_config::get_config, app_state::AppState},
    repos::thoughts_repo::ThoughtsRepo,
    web_api::handlers::create_thought,
};
use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
    sync::Arc,
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::log;

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    // Logging init.
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            format!(
                "{},hyper=info,mio=info,sqlx=warn,tower_http=warn",
                opt.log_level
            ),
        )
    }

    tracing_subscriber::fmt::init();

    // TODO: tbd its usage.
    let _app_cfg = get_config().expect("Failed to load the app config.");

    let app_state = AppState::new(ThoughtsRepo::new());

    let routes = routes(app_state);

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));
    log::info!("Listening for requests on http://{} ...", sock_addr);

    axum::Server::bind(&sock_addr)
        .serve(routes.into_make_service())
        .await
        .expect("Unable to start server");
}

fn routes(state: AppState) -> Router {
    let tracing_layer = TraceLayer::new_for_http();
    let cors_layer = CorsLayer::new().allow_origin(Any);

    Router::new()
        .route("/thoughts", post(create_thought))
        .layer(tracing_layer)
        .layer(cors_layer)
        .with_state(Arc::new(state))
}

#[derive(Parser, Debug)]
#[clap(
    name = "server",
    about = "The server side of Fullstack Rust RealWorld App project."
)]
struct Opt {
    /// The HTTP listening address.
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: String,

    /// The HTTP listening port.
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,

    /// The logging level.
    #[clap(short = 'l', long = "log", default_value = "info")]
    log_level: String,
}
