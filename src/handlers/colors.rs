use axum::{Json, extract::State, http::StatusCode};

use crate::{
  models::{AddColorRequest, ColorsResponse},
  services::colors as color_service,
  state::AppState,
};

pub async fn get_colors(State(state): State<AppState>) -> Json<ColorsResponse> {
  let colors = color_service::get_colors(&state.colors);

  Json(ColorsResponse { colors })
}

pub async fn add_color(
  State(state): State<AppState>,
  Json(payload): Json<AddColorRequest>,
) -> (StatusCode, Json<ColorsResponse>) {
  let colors = color_service::add_color(&state.colors, payload.color);

  (StatusCode::CREATED, Json(ColorsResponse { colors }))
}
