use server::handler;
use database::connection_pool;

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, AllowedHeaders};


#[get("/")]
fn cors<'a>() -> &'a str {
	"Hello CORS"
}

pub fn create_routes() {
	let (allowed_origins, failed_origins) = AllowedOrigins::some(&["http://localhost:8080"]);
	assert!(failed_origins.is_empty());

	// You can also deserialize this
	let options = rocket_cors::Cors {
		allowed_origins: allowed_origins,
		allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
		allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
		allow_credentials: true,
		..Default::default()
	};

	rocket::ignite()
		.manage(connection_pool::init_pool())
		.mount(
			"/",
			routes![cors]
		)
		.mount(
			"/objects",
			routes![
				handler::all,
				handler::get,
				handler::post,
				handler::put,
				handler::delete
			],
		)
		.mount(
			"/",
			routes![
				handler::auth_post
			],
		)
		.attach(options)
		.launch();
}
