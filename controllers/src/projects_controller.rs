use tide::{Body, Request, Response, Result};

use models::db::projects::{NewProject, Project};

/*

 */
pub async fn index(_req: Request<()>) -> Result<Body> {
	let projects = vec![
		Project {
      id: 1,
			name: "Gandalf".to_string(),
		},
		Project {
      id: 2,
			name: "Oz".to_string(),
		}
	];

	Ok(Body::from_json(&projects)?)
}

/*

 */
pub async fn create(mut req: Request<()>) -> Result {
	let params: NewProject = req.body_json().await?;

  let project = Project {
    id: 3,
    name: params.name
  };

  let response =
    Response::builder(201)
      .body(Body::from_json(&project)?)
      .build();

  Ok(response)
}
