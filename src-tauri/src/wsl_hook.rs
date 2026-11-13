use axum::{routing::post, Router, Json, extract::State};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tauri::{AppHandle, Emitter};

#[derive(Serialize, Deserialize, Clone)]
pub struct NotifyPayload {
    pub source: String,
    pub duration_seconds: u64,
    pub exit_code: i32,
}

async fn handle_notify(
    State(app): State<AppHandle>,
    Json(payload): Json<NotifyPayload>
) -> &'static str {
    let _ = app.emit("async_task_complete", payload);
    "OK"
}

pub fn start_server(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let router = Router::new()
            .route("/notch/notify", post(handle_notify))
            .with_state(app);
        let addr = SocketAddr::from(([127, 0, 0, 1], 9182));
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener, router).await.unwrap();
    });
}
