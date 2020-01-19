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
        let _ = rel
            .to_relation(connection) // ignore bad formated records
            .map(|r| vec.push(r))
            .map_err(|e| {
                let hex_string = GenericHash::from_bytes(&hash).to_string();
                error!(
                    "[query - all]: Bad formated relation [{}]: {:?}",
                    hex_string, e
                );
            });
    }
    Ok(vec)
}

pub fn get(hash: GenericHash, connection: &PgConnection) -> QueryResult<Relation> {
    let return_db_rel = relations::table
        .find(&hash.to_vec())
        .get_result::<DatabaseRelation>(connection)?;

    let relation = return_db_rel.to_relation(connection)?;
    Ok(relation)
}

pub fn insert(
    database_relation: DatabaseRelation,
    connection: &PgConnection,
) -> QueryResult<Relation> {
    let return_db_rel = diesel::insert_into(relations::table)
        .values(&database_relation)
        .get_result::<DatabaseRelation>(connection)?;

    let relation = return_db_rel.to_relation(connection)?;
    Ok(relation)
}

pub fn delete(hash: GenericHash, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(relations::table.find(hash.to_vec())).execute(connection)
}
