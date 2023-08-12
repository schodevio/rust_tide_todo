use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Project {
  pub id: u32,
  pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct NewProject {
  pub name: String,
}
