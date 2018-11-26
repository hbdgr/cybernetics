use server::handler;
use database::connection_pool;


pub fn create_routes() {
	rocket::ignite()
		.manage(connection_pool::init_pool())
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
		).launch();
}
