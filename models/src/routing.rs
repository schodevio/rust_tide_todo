use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default)]
pub struct HelloQuery {
	pub name: String,
}

impl Default for HelloQuery {
	fn default() -> Self {
		Self { name: "World".to_string() }
	}
}
