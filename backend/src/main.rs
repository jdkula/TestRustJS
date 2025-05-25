mod proto;
use std::{collections::HashMap, sync::LazyLock};

use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
};
use axum_extra::protobuf::Protobuf;
use std::sync::Mutex;
use tower_http::cors::CorsLayer;

static DB: LazyLock<Mutex<HashMap<String, i64>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/accumulate", post(accumulate))
        .route("/finish", post(finish))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn accumulate(
    Protobuf(payload): Protobuf<proto::api::AccumulateRequest>,
) -> (StatusCode, Protobuf<proto::api::AccumulateResponse>) {
    let mut db = DB.lock().unwrap();
    let curr = db.get(&payload.id.clone()).copied();
    let status = if curr.is_none() {
        StatusCode::CREATED
    } else {
        StatusCode::OK
    };

    let value = curr.unwrap_or(0i64);
    db.insert(payload.id.clone(), value + payload.number);

    let mut result = proto::api::AccumulateResponse::default();
    result.id = payload.id.clone();
    result.so_far = value;

    (status, Protobuf(result))
}

async fn finish(
    Protobuf(payload): Protobuf<proto::api::FinishPipelineRequest>,
) -> (StatusCode, Protobuf<proto::api::FinishResponse>) {
    let mut db = DB.lock().unwrap();
    let curr = db.get(&payload.id.clone()).copied();
    if curr.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Protobuf(proto::api::FinishResponse::default()),
        );
    };

    db.remove(&payload.id.clone());

    let mut result = proto::api::FinishResponse::default();
    result.id = payload.id.clone();
    result.sum = curr.unwrap();

    (StatusCode::OK, Protobuf(result))
}
