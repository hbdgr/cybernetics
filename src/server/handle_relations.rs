use database::connection_pool::DbConn;
use std::env;

use crypto::hash::GenericHash;
use database::relation::DatabaseRelation;
use database::relation_queries;
use primitives::relation::{Relation, RelationBase};

use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use server::router::error_status;

#[post("/", format = "application/json", data = "<relation_base>")]
pub fn post(
    relation_base: Json<RelationBase>,
    connection: DbConn,
) -> Result<status::Created<Json<Relation>>, Status> {
    let rb_inner = relation_base.into_inner();
    let database_rel = DatabaseRelation::from_relation_base(rb_inner);

    relation_queries::insert(database_rel, &connection)
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
#[put("/<hash>", format = "application/json", data = "<relation_base>")]
pub fn put(
    hash: String,
    relation_base: Json<RelationBase>,
    connection: DbConn,
) -> Result<status::Created<Json<Relation>>, Status> {
    let rb_inner = relation_base.into_inner();
    let new_hash = rb_inner.hash().unwrap();

    // check if relation already exist
    if let Ok(_) = relation_queries::get(new_hash, &connection) {
        return Err(Status::Conflict);
    }

    let ghash = GenericHash::from_hex(&hash);
    let database_rel = DatabaseRelation::from_relation_base(rb_inner);
    relation_queries::insert(database_rel, &connection)
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
