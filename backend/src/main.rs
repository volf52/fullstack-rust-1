mod db;
mod routes;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::Router;
use axum_extra::routing::SpaRouter;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug,hyper=info,mio=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    db::init_db_and_migrate().await;

    let api_router = routes::create_api_router();
    let app = Router::new()
        .nest("/api", api_router)
        .merge(SpaRouter::new("/", "./dist"))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let sock_addr = SocketAddr::from((IpAddr::V4(Ipv4Addr::LOCALHOST), 3001));
    log::info!("Listening on http://{}", sock_addr);
    axum::Server::bind(&sock_addr)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start server");
}
