use crypto::hash::GenericHash;
use primitives::object::Object;

use database::object::{InsertableObject, QueryableObject};
use database::schema::objects;
use diesel;
use diesel::prelude::*;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Object>> {
    let queryable_vec = objects::table.load::<QueryableObject>(&*connection)?;

    let mut vec = Vec::new();
    for ins in queryable_vec {
        let id = ins.id.clone();
        let obj = match Object::from_queryable_object(ins) {
            Ok(obj) => obj,
            Err(err) => {
                error!("[query - all]: Bad formated object [{:?}]: {:?}", id, err);
                continue;
            }
        };
        vec.push(obj)
    }
    Ok(vec)
}

pub fn get(id: GenericHash, connection: &PgConnection) -> QueryResult<Object> {
    let queryable = objects::table
        .find(&id.to_vec())
        .get_result::<QueryableObject>(connection)?;

    let obj = Object::from_queryable_object(queryable).unwrap();
    Ok(obj)
}

pub fn insert(object: InsertableObject, connection: &PgConnection) -> QueryResult<Object> {
    let queryable = diesel::insert_into(objects::table)
        .values(&object)
        .get_result(connection)?;

    let obj = Object::from_queryable_object(queryable).unwrap();
    Ok(obj)
}

pub fn update(object: QueryableObject, connection: &PgConnection) -> QueryResult<Object> {
    let queryable = diesel::update(objects::table.find(object.id.to_vec()))
        .set(&object)
        .get_result(connection)?;

    let obj = Object::from_queryable_object(queryable).unwrap();
    Ok(obj)
}

pub fn delete(id: GenericHash, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(objects::table.find(id.to_vec())).execute(connection)
}
