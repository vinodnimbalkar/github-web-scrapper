use axum::{routing::get, Router};
use dotenv::dotenv;
mod handlers;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(handlers::root))
        // `GET /` goes to `handler`
        .route("/:username/:year", get(handlers::handler));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    dotenv().ok();
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("Invalid PORT");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
