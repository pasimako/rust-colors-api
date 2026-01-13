use std::sync::{Arc, Mutex};

use tokio::net::TcpListener;

use colors_api::app;
use colors_api::state::AppState;

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 3000;

#[tokio::main]
async fn main() {
  let state = AppState {
    colors: Arc::new(Mutex::new(vec![
      String::from("RED"),
      String::from("GREEN"),
      String::from("BLUE"),
    ])),
  };

  let app = app::create_app(state);

  let listener = TcpListener::bind((DEFAULT_HOST, DEFAULT_PORT))
    .await
    .unwrap();

  println!(
    "Server listening on http://{}:{}",
    DEFAULT_HOST, DEFAULT_PORT
  );

  axum::serve(listener, app).await.unwrap();
}
