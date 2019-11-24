use database::connection_pool::DbConn;
use std::env;

use database::relation::{InsertableRelation, Relation};
use database::relation_queries;

use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use server::router::error_status;

#[post("/", format = "application/json", data = "<relation>")]
pub fn post(
    relation: Json<InsertableRelation>,
    connection: DbConn,
) -> Result<status::Created<Json<Relation>>, Status> {
    relation_queries::insert(relation.into_inner(), &connection)
        .map(|relation| relation_created(relation))
        .map_err(|error| error_status(error))
}

pub fn relation_created(relation: Relation) -> status::Created<Json<Relation>> {
    let host = env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set");
    let port = env::var("ROCKET_PORT").expect("ROCKET_PORT must be set");

    status::Created(
        format!(
            "{host}:{port}/relations/{id}",
            host = host,
            port = port,
            id = relation.id
        )
        .to_string(),
        Some(Json(relation)),
    )
}
