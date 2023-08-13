use tide::{Body, Request, Response, Result};

use models::db::projects::{NewProject, Project};
use shared::db::Connection;

/*

 */
pub async fn index(_req: Request<()>) -> Result {
  let conn = Connection::init().await?;
  let projects: Vec<Project> = conn.select("projects").await?;

  let body = Body::from_json(&projects)?;

	Ok(
    Response::builder(200)
      .body(body)
      .build()
  )
}

/*

 */
pub async fn create(mut req: Request<()>) -> Result {
  let params: NewProject = req.body_json().await?;

  let conn = Connection::init().await?;
  let project: Project = conn.create("projects").content(params).await?;

  let body = Body::from_json(&project)?;

  Ok(
    Response::builder(201)
      .body(body)
      .build()
  )
}

/*

 */
pub async fn update(mut req: Request<()>) -> Result {
  let params: NewProject = req.body_json().await?;
  let project_id: &str = req.param("id").expect("Missing Project ID");

  let conn = Connection::init().await?;
  let project: Project = conn.update(("projects", project_id)).merge(params).await?;

  let body = Body::from_json(&project)?;

  Ok(
    Response::builder(200)
      .body(body)
      .build()
  )
}

/*

 */
pub async fn destroy(req: Request<()>) -> Result {
  let project_id: &str = req.param("id").expect("Missing Project ID");

  let conn = Connection::init().await?;
  let _project: Option<Project> = conn.delete(("projects", project_id)).await?;

  Ok(
    Response::builder(204).build()
  )
}
