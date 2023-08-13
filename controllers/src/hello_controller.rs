use tide::{Request, Result};

use models::routing;

/*

 */
pub async fn hello(_req: Request<()>) -> Result<String> {
	Ok("Hello ToDo App from Docker".to_string())
}

/*

 */
pub async fn hello_name(req: Request<()>) -> Result<String> {
	let name = req.param("name").unwrap_or("World");

	Ok(format!("Hello {}!", name))
}

/*

 */
pub async fn hello_query(req: Request<()>) -> Result<String> {
	let query: routing::HelloQuery = req.query()?;

	Ok(format!("Hello {}!", query.name))
}
