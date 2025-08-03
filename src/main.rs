use axum::body::Body;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Router, response::Html, response::Response};
use nix_stat::get_system_health;
use std::net::SocketAddr;

async fn index() -> Html<&'static str> {
    Html(include_str!("../templates/index.html"))
}

async fn stream_handler() -> Response {
    let sse_stream = async_stream::stream! {
        loop {
            let health = get_system_health();
            let json = serde_json::to_string(&health).unwrap();
            yield Ok::<_, std::io::Error>(format!("data: {}\n\n", json));
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    };
    let body = Body::from_stream(sse_stream);
    Response::builder()
        .header("Content-Type", "text/event-stream")
        .header("Cache-Control", "no-cache")
        .header("Connection", "keep-alive")
        .body(body)
        .unwrap()
}

async fn logs_handler(Path(unit): Path<String>) -> (StatusCode, String) {
    match nix_stat::get_journal_logs(&unit) {
        Ok(logs) => (StatusCode::OK, logs),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get logs: {}", e),
        ),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/stream", get(stream_handler))
        .route("/api/logs/{unit}", get(logs_handler));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Live health server running on http://{}", addr);
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}
