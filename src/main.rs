use axum::{response::sse, routing::get, Router};
use pages::{
    index::index,
    twiddle::{process_twiddle, twiddle, upload_twiddle},
};
use tokio::net::TcpListener;
use tracing::{event, instrument, Level};

mod db;
mod pages;

#[tokio::main]
#[instrument]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(index))
        .route("/twiddle", get(twiddle).post(upload_twiddle))
        .route("/twiddle/:filename/process", get(process_twiddle));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    event!(Level::INFO, "Server starting on 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
