use database::connection_pool::DbConn;
use std::env;

use database::object::{InsertableObject, QueryableObject};
use database::object_queries;
use primitives::object::Object;

use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use server::router::error_status;

#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<Object>>, Status> {
    object_queries::all(&connection)
        .map(|objects| Json(objects))
        .map_err(|error| error_status(error))
}

/*
#[options("/auth", format = "text/html")]
fn auth_post() -> status::NoContent {
    println!("Got auth_data!");
     status::NoContent
}
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthData {
    key: String,
    password: String,
}

#[post("/auth", format = "application/json", data = "<auth_data>")]
pub fn auth_post(auth_data: Json<AuthData>) -> Status {
    println!("Got auth_data! {:?}", auth_data);
    Status::NoContent
}

#[get("/<id>")]
pub fn get(id: i64, connection: DbConn) -> Result<Json<Object>, Status> {
    object_queries::get(id, &connection)
        .map(|object| Json(object))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<object>")]
pub fn post(
    object: Json<InsertableObject>,
    connection: DbConn,
) -> Result<status::Created<Json<Object>>, Status> {
    object_queries::insert(object.into_inner(), &connection)
        .map(|object| object_created(object))
        .map_err(|error| error_status(error))
}

#[put("/<id>", format = "application/json", data = "<object>")]
pub fn put(
    id: i64,
    object: Json<InsertableObject>,
    connection: DbConn,
) -> Result<Json<Object>, Status> {
    let queryable = QueryableObject::from_insertable_object(id, object.into_inner());

    object_queries::update(queryable, &connection)
        .map(|object| Json(object))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i64, connection: DbConn) -> Result<Status, Status> {
    match object_queries::get(id, &connection) {
        Ok(_) => object_queries::delete(id, &connection)
            .map(|_| Status::NoContent)
            .map_err(|error| error_status(error)),
        Err(error) => Err(error_status(error)),
    }
}

pub fn object_created(object: Object) -> status::Created<Json<Object>> {
    let host = env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set");
    let port = env::var("ROCKET_PORT").expect("ROCKET_PORT must be set");

    status::Created(
        format!(
            "{host}:{port}/objects/{id}",
            host = host,
            port = port,
            id = object.id
        )
        .to_string(),
        Some(Json(object)),
    )
}
