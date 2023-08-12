use controllers::{
	hello_controller,
	projects_controller
};

const HOST: &str = "127.0.0.1";
const PORT: &str = "1234";

#[async_std::main]
async fn main() -> tide::Result<()> {
	femme::start();

	let mut app = tide::new();
	app.with(tide::log::LogMiddleware::new());

	// Hello
	app.at("/").get(hello_controller::hello);
	app.at("/hello/:name").get(hello_controller::hello_name);
	app.at("/hello").get(hello_controller::hello_query);

	// Projects
	app.at("/api/v1").nest({
		let mut api = tide::new();

		api.at("/projects").nest({
			let mut projects = tide::new();

			projects.at("/").get(projects_controller::index);
			projects.at("/").post(projects_controller::create);

			projects
		});

		api
	});

	// Static Files
	app.at("/public").serve_dir("./public")?;

	// App run
	app.listen(format!("{}:{}", HOST, PORT)).await?;

	Ok(())
}
