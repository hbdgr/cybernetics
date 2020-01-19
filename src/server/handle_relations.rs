use database::connection_pool::DbConn;
use std::env;

use crypto::hash::GenericHash;
use database::relation::DatabaseRelationBase;
use database::relation_queries;
use primitives::relation::Relation;

use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use server::router::error_status;

#[post("/", format = "application/json", data = "<database_relation_base>")]
pub fn post(
    database_relation_base: Json<DatabaseRelationBase>,
    connection: DbConn,
) -> Result<status::Created<Json<Relation>>, Status> {
    let drb_inner = database_relation_base.into_inner();
    let rb_inner = match drb_inner.to_database_relation(&connection) {
        Ok(rb) => rb,
        Err(err) => return Err(error_status(err)),
    };

    let new_hash = rb_inner.hash();

    // check if relation already exist
    if let Ok(_) = relation_queries::get(new_hash, &connection) {
        return Err(Status::Conflict);
    }

    relation_queries::insert(rb_inner, &connection)
        .map(|relation| relation_created(relation))
        .map_err(|error| error_status(error))
}

pub fn relation_created(relation: Relation) -> status::Created<Json<Relation>> {
    let host = env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set");
    let port = env::var("ROCKET_PORT").expect("ROCKET_PORT must be set");

    status::Created(
        format!(
            "{host}:{port}/relations/{hash}",
            host = host,
            port = port,
            hash = relation.hash
        )
        .to_string(),
        Some(Json(relation)),
    )
}

// put object with hash means create new and delete previous
#[put(
    "/<hash>",
    format = "application/json",
    data = "<database_relation_base>"
)]
pub fn put(
    hash: String,
    database_relation_base: Json<DatabaseRelationBase>,
    connection: DbConn,
) -> Result<status::Created<Json<Relation>>, Status> {
    let drb_inner = database_relation_base.into_inner();
    let rb_inner = match drb_inner.to_database_relation(&connection) {
        Ok(rb) => rb,
        Err(err) => return Err(error_status(err)),
    };

    let new_hash = rb_inner.hash();

    // check if relation already exist
    if let Ok(_) = relation_queries::get(new_hash, &connection) {
        return Err(Status::Conflict);
    }

    let ghash = GenericHash::from_hex(&hash);
    relation_queries::insert(rb_inner, &connection)
        .map_err(|err| error_status(err))
        .map(|relation| {
            let _ = relation_queries::delete(ghash, &connection).map_err(|err| error_status(err));
            relation_created(relation)
        })
}

#[get("/<hash>")]
pub fn get(hash: String, connection: DbConn) -> Result<Json<Relation>, Status> {
    relation_queries::get(GenericHash::from_hex(&hash), &connection)
        .map(|relation| Json(relation))
        .map_err(|error| error_status(error))
}

#[get("/")]
pub fn all(connection: DbConn) -> Result<Json<Vec<Relation>>, Status> {
    relation_queries::all(&connection)
        .map(|relations| Json(relations))
        .map_err(|error| error_status(error))
}

#[delete("/<hash>")]
pub fn delete(hash: String, connection: DbConn) -> Result<Status, Status> {
    let ghash = GenericHash::from_hex(&hash);

    if let Err(err) = relation_queries::get(ghash.clone(), &connection) {
        return Err(error_status(err));
    }

    relation_queries::delete(ghash, &connection)
        .map(|_| Status::NoContent)
        .map_err(|err| error_status(err))
}
