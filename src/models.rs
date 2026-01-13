use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ColorsResponse {
  pub colors: Vec<String>,
}

#[derive(Deserialize)]
pub struct AddColorRequest {
  pub color: String,
}
