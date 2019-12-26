use crypto::hash::GenericHash;
use primitives::relation::Relation;

use database::relation::DatabaseRelation;
use database::schema::relations;
use diesel;
use diesel::prelude::*;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Relation>> {
    let return_vec = relations::table.load::<DatabaseRelation>(&*connection)?;

    let mut vec = Vec::new();
    for rel in return_vec {
        let hash = rel.hash.clone();
        let _ = Relation::from_database_relation(rel) // ignore bad formated records
            .map(|r| vec.push(r))
            .map_err(|e| {
                error!("[query - all]: Bad formated relation [{:?}]: {:?}", hash, e);
            });
    }
    Ok(vec)
}

pub fn get(hash: GenericHash, connection: &PgConnection) -> QueryResult<Relation> {
    let return_relation = relations::table
        .find(&hash.to_vec())
        .get_result::<DatabaseRelation>(connection)?;

    let relation = Relation::from_database_relation(return_relation).unwrap();
    Ok(relation)
}

pub fn insert(
    database_relation: DatabaseRelation,
    connection: &PgConnection,
) -> QueryResult<Relation> {
    let return_relation = diesel::insert_into(relations::table)
        .values(&database_relation)
        .get_result(connection)?;

    let relation = Relation::from_database_relation(return_relation).unwrap();
    Ok(relation)
}

pub fn delete(hash: GenericHash, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(relations::table.find(hash.to_vec())).execute(connection)
}
