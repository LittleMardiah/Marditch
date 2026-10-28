use axum::{routing::post, Router, Json};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Deserialize)]
pub struct NotifyPayload {
    source: String,
    duration_seconds: u64,
    exit_code: i32,
}

async fn handle_notify(Json(payload): Json<NotifyPayload>) -> &'static str {
    println!("[WSL Hook] Async task done: {} - {}s (exit: {})", payload.source, payload.duration_seconds, payload.exit_code);
    "OK"
}

pub fn start_server() {
    tauri::async_runtime::spawn(async move {
        let app = Router::new().route("/notch/notify", post(handle_notify));
        let addr = SocketAddr::from(([127, 0, 0, 1], 9182));
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        println!("[WSL Hook] Listening on {}", addr);
        axum::serve(listener, app).await.unwrap();
    });
}
