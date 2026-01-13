use std::sync::{Arc, Mutex};

pub type ColorsData = Arc<Mutex<Vec<String>>>;

#[derive(Clone)]
pub struct AppState {
  pub colors: ColorsData,
}
