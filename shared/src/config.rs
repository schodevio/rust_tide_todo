use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Envs {
  pub db_user: String,
  pub db_password: String,

  pub db_host: String,
  pub db_port: String,

  pub db_namespace: String,
  pub db_database: String,
}

impl Envs {
  pub fn init() -> Self {
    dotenvy::dotenv()
      .expect("ConfigError: enable to load .env file.");

    envy::from_env::<Envs>()
      .expect("ConfigError: unable to map envs.")
  }
}
