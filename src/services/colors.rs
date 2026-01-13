use crate::state::ColorsData;

pub fn get_colors(colors: &ColorsData) -> Vec<String> {
  colors.lock().unwrap().clone()
}

pub fn add_color(colors: &ColorsData, color: String) -> Vec<String> {
  let mut colors = colors.lock().unwrap();
  colors.push(color.to_uppercase());

  colors.clone()
}
