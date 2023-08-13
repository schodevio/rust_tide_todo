use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Deserialize, Serialize)]
pub struct Project {
  pub id: Thing,
  pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct NewProject {
  pub name: String,
}
