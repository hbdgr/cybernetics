use database::connection_pool::DbConn;
use diesel::result::Error;
use std::env;
use database::object_queries;
use database::object::Object;
use rocket::http::Status;
use rocket::response::{Failure, status};
use rocket_contrib::Json;


#[get("/")]
fn all(connection: DbConn) -> Result<Json<Vec<Object>>, Failure> {
	object_queries::all(&connection)
		.map(|objects| Json(objects))
		.map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Failure {
	Failure(match error {
		Error::NotFound => Status::NotFound,
		_ => Status::InternalServerError
	})
}
/*
#[options("/auth", format = "text/html")]
fn auth_post() -> status::NoContent {
	println!("Got auth_data!");
	 status::NoContent
}
*/
#[derive(Serialize, Deserialize, Debug)]
struct AuthData {
	key: String,
	password: String
}

#[post("/auth", format = "application/json", data = "<auth_data>")]
fn auth_post(auth_data: Json<AuthData>) -> status::NoContent {
	println!("Got auth_data! {:?}", auth_data);
	status::NoContent
}

#[get("/<id>")]
fn get(id: i32, connection: DbConn) -> Result<Json<Object>, Failure> {
	object_queries::get(id, &connection)
		.map(|object| Json(object))
		.map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<object>")]
fn post(object: Json<Object>, connection: DbConn) -> Result<status::Created<Json<Object>>, Failure> {
	object_queries::insert(object.into_inner(), &connection)
		.map(|object| object_created(object))
		.map_err(|error| error_status(error))
}

fn object_created(object: Object) -> status::Created<Json<Object>> {
	let host = env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set");
	let port = env::var("ROCKET_PORT").expect("ROCKET_PORT must be set");

	status::Created(
		format!("{host}:{port}/objects/{id}", host = host, port = port, id = object.id).to_string(),
		Some(Json(object)))
}

#[put("/<id>", format = "application/json", data = "<object>")]
fn put(id: i32, object: Json<Object>, connection: DbConn) -> Result<Json<Object>, Failure> {
	object_queries::update(id, object.into_inner(), &connection)
		.map(|object| Json(object))
		.map_err(|error| error_status(error))
}

#[delete("/<id>")]
fn delete(id: i32, connection: DbConn) -> Result<status::NoContent, Failure> {
	match object_queries::get(id, &connection) {
		Ok(_) => object_queries::delete(id, &connection)
			.map(|_| status::NoContent)
			.map_err(|error| error_status(error)),
		Err(error) => Err(error_status(error))
	}
}
