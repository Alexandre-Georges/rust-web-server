use axum::{
  http::StatusCode,
  Json, Router,
  routing::{get, post},
};
use std::{net::SocketAddr};
use tracing::{Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
  let subscriber = FmtSubscriber::builder()
    .with_max_level(Level::INFO)
    .finish();

  tracing::subscriber::set_global_default(subscriber)
    .expect("setting default subscriber failed");

  let app = Router::new()
    .route("/", get(root))
    .route("/users", post(create_user));

  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  tracing::info!("listening on {}", addr);
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn root() -> &'static str {
  "Hello, World!"
}

struct Payload<'a> {
  username: &'a str,
}

fn validate_payload<'a>(payload: &'a serde_json::Value) -> Option<Payload<'a>> {
  if !payload.is_object() {
    return Option::None;
  }
  
  let payload_object = payload.as_object();
  if !payload_object.is_some() {
    return Option::None; 
  }

  let payload_some = payload_object.unwrap();
  let username = payload_some.get("username");
  if !username.is_some() {
    return Option::None;
  }
  
  let username_some = username.unwrap();
  if !username_some.is_string() {
    return Option::None;
  }

  let username_string = username_some.as_str().unwrap();
  Option::Some(Payload {
    username: username_string,
  })
}

async fn create_user(
  Json(payload): Json<serde_json::Value>,
) -> (StatusCode, Json<serde_json::Value>) {
  let result = validate_payload(&payload);
  if result.is_none() {
    return (StatusCode::BAD_REQUEST, Json(serde_json::Value::Null));
  }
  let payload = result.unwrap(); 
  tracing::info!("{}", payload.username);
  (StatusCode::CREATED, Json(Into::into(payload.username)))
}
