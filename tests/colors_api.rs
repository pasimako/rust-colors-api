use std::sync::{Arc, Mutex};

use axum::body::{Body, to_bytes};
use axum::http::{Request, Response, StatusCode};
use serde::Deserialize;
use tower::ServiceExt;

use colors_api::{app::create_app, state::AppState};

#[derive(Deserialize)]
struct ColorsResponse {
  colors: Vec<String>,
}

#[tokio::test]
async fn get_colors_works() {
  let state = AppState {
    colors: Arc::new(Mutex::new(vec![
      String::from("RED"),
      String::from("GREEN"),
      String::from("BLUE"),
    ])),
  };

  let app = create_app(state);

  let response = app.oneshot(get("/colors")).await.unwrap();

  assert_eq!(response.status(), StatusCode::OK);

  let body: ColorsResponse = json_body(response).await;
  assert_eq!(body.colors, vec!["RED", "GREEN", "BLUE"]);
}

#[tokio::test]
async fn post_color_works() {
  let state = AppState {
    colors: Arc::new(Mutex::new(vec![
      String::from("RED"),
      String::from("GREEN"),
    ])),
  };

  let app = create_app(state);

  let response = app
    .clone()
    .oneshot(post_json("/colors", serde_json::json!({ "color": "BLUE" })))
    .await
    .unwrap();

  assert_eq!(response.status(), StatusCode::CREATED);

  let body: ColorsResponse = json_body(response).await;
  assert_eq!(body.colors, vec!["RED", "GREEN", "BLUE"]);
}

// Test helpers

fn get(uri: &str) -> Request<Body> {
  Request::builder().uri(uri).body(Body::empty()).unwrap()
}

fn post_json(uri: &str, json: serde_json::Value) -> Request<Body> {
  Request::builder()
    .method("POST")
    .uri(uri)
    .header("content-type", "application/json")
    .body(Body::from(json.to_string()))
    .unwrap()
}

async fn json_body<T: serde::de::DeserializeOwned>(
  response: Response<Body>,
) -> T {
  let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
  serde_json::from_slice(&bytes).unwrap()
}
