use database::relation::{InsertableRelation, Relation};
use database::schema::relations;
use diesel;
use diesel::prelude::*;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Relation>> {
    relations::table.load::<Relation>(&*connection)
}

pub fn get(id: i64, connection: &PgConnection) -> QueryResult<Relation> {
    let rel = relations::table
        .find(id)
        .get_result::<Relation>(connection)?;

    Ok(rel)
}

pub fn insert(relation: InsertableRelation, connection: &PgConnection) -> QueryResult<Relation> {
    let rel = diesel::insert_into(relations::table)
        .values(&relation)
        .get_result(connection)?;

    Ok(rel)
}

pub fn update(id: i64, relation: Relation, connection: &PgConnection) -> QueryResult<Relation> {
    let insertable = InsertableRelation::from_relation(relation);

    let rel = diesel::update(relations::table.find(id))
        .set(&insertable)
        .get_result(connection)?;

    Ok(rel)
}

pub fn delete(id: i64, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(relations::table.find(id)).execute(connection)
}
