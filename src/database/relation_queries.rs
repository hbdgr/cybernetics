use database::relation::InsertableRelation;
use database::schema::relations;
use diesel;
use diesel::prelude::*;
use primitives::relation::Relation

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Relation>> {
    relations::table.load::<Relation>(&*connection)?
}

pub fn get(id: i64, connection: &PgConnection) -> QueryResult<Relation> {
    relations::table
        .find(id)
        .get_result::<QueryableObject>(connection)?;
}

pub fn insert(relation: InsertableRelation, connection: &PgConnection) -> QueryResult<Relation> {
    diesel::insert_into(relations::table)
        .values(&relation)
        .get_result(connection)?;
}

pub fn update(id: i64, relation: Relation, connection: &PgConnection) -> QueryResult<Relation> {
    let insertable = InsertableRelation::from_relation(relation);

    diesel::update(relations::table.find(id))
        .set(&insertable)
        .get_result(connection)?;
}

pub fn delete(id: i64, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(relations::table.find(id)).execute(connection)
}
