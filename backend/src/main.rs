mod proto;
use std::{collections::{hash_map::Entry::{Occupied, Vacant}, HashMap}, sync::LazyLock};

use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
};
use axum_extra::protobuf::Protobuf;
use tokio::sync::Mutex;
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

fn test(v: &mut i64) {
    *v += 1;
}

async fn accumulate(
    Protobuf(payload): Protobuf<proto::api::AccumulateRequest>,
) -> (StatusCode, Protobuf<proto::api::AccumulateResponse>) {
    let mut status = StatusCode::OK;
    let value = {
        let mut db = DB.lock().await;

        match db.entry(payload.id.clone()) {
            Occupied(mut occupied_entry) => {
                let val_ref = occupied_entry.get_mut();
                *val_ref += payload.number;
                *val_ref
            }
            Vacant(vacant_entry) => {
                status = StatusCode::CREATED;
                *vacant_entry.insert(payload.number)
            }
        }
    };

    let mut result = proto::api::AccumulateResponse::default();
    result.id = payload.id;
    result.so_far = value;

    (status, Protobuf(result))
}

async fn finish(
    Protobuf(payload): Protobuf<proto::api::FinishPipelineRequest>,
) -> (StatusCode, Protobuf<proto::api::FinishResponse>) {
    let mut db = DB.lock().await;
    let id = payload.id.clone();

    let curr = {
        let curr = db.remove(&id);
        if curr.is_none() {
            return (
                StatusCode::NOT_FOUND,
                Protobuf(proto::api::FinishResponse::default()),
            );
        };
        curr.unwrap()
    };

    let mut result = proto::api::FinishResponse::default();
    result.id = id;
    result.sum = curr;

    (StatusCode::OK, Protobuf(result))
}
