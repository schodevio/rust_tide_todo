use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};

use crate::config::Envs;

#[derive(Debug)]
pub struct Connection {}

impl Connection {
  pub async fn init() -> Result<Surreal<Client>, Error> {
    let envs = Envs::init();

    let url = format!("{}:{}", &envs.db_host, &envs.db_port);

    let client = Surreal::new::<Ws>(url).await?;

    let user = Root {
      username: &envs.db_user,
      password: &envs.db_password,
    };

    client
      .signin(user)
      .await?;

    client
      .use_ns(envs.db_namespace)
      .use_db(envs.db_database)
      .await?;

    Ok(client)
  }
}
