use database::connection_pool::DbConn;
use std::env;

use crypto::hash::GenericHash;
use database::object::DatabaseObject;
use database::object_queries;
use primitives::object::{Content, Object};

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

#[get("/<hash>")]
pub fn get(hash: String, connection: DbConn) -> Result<Json<Object>, Status> {
    object_queries::get(GenericHash::from_hex(&hash), &connection)
        .map(|object| Json(object))
        .map_err(|error| error_status(error))
}

#[post("/", format = "application/json", data = "<content>")]
pub fn post(
    content: Json<Content>,
    connection: DbConn,
) -> Result<status::Created<Json<Object>>, Status> {
    let c_inner = content.into_inner();

    let obj_exist = object_queries::get(c_inner.hash().unwrap(), &connection);
    if obj_exist.is_ok() {
        return Err(Status::Conflict);
    }

    object_queries::insert(DatabaseObject::from_content(c_inner), &connection)
        .map(|object| object_created(object))
        .map_err(|error| error_status(error))
}

// put object with hash means create new and delete previous
#[put("/<hash>", format = "application/json", data = "<content>")]
pub fn put(
    hash: String,
    content: Json<Content>,
    connection: DbConn,
) -> Result<status::Created<Json<Object>>, Status> {
    let c_inner = content.into_inner();

    let obj_exist = object_queries::get(c_inner.hash().unwrap(), &connection);
    if obj_exist.is_ok() {
        return Err(Status::Conflict);
    }

    let obj = match object_queries::insert(DatabaseObject::from_content(c_inner), &connection) {
        Ok(obj) => obj,
        Err(error) => return Err(error_status(error)),
    };
    match object_queries::delete(GenericHash::from_hex(&hash), &connection) {
        Ok(s) => s,
        Err(error) => return Err(error_status(error)),
    };

    Ok(object_created(obj))
}

// real delete is possible only for not published objects
#[delete("/<hash>")]
pub fn delete(hash: String, connection: DbConn) -> Result<Status, Status> {
    match object_queries::get(GenericHash::from_hex(&hash), &connection) {
        Ok(_) => object_queries::delete(GenericHash::from_hex(&hash), &connection)
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
            "{host}:{port}/objects/{hash}",
            host = host,
            port = port,
            hash = object.hash,
        )
        .to_string(),
        Some(Json(object)),
    )
}
