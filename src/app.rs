use axum::Router;

use crate::{routes, state::AppState};

pub fn create_app(state: AppState) -> Router {
  Router::new()
    .nest("/colors", routes::colors::router())
    .with_state(state)
}
