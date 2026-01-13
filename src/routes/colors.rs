use axum::{
  Router,
  routing::{get, post},
};

use crate::{
  handlers::colors::{add_color, get_colors},
  state::AppState,
};

pub fn router() -> Router<AppState> {
  Router::new()
    .route("/", get(get_colors))
    .route("/", post(add_color))
}
