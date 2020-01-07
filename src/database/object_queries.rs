use crypto::hash::GenericHash;
use primitives::object::{Content, Object};

use database::connection_pool;
use database::object::DatabaseObject;
use database::schema::objects;
use diesel;
use diesel::prelude::*;
use std::ops::Deref;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Object>> {
    let return_vec = objects::table.load::<DatabaseObject>(&*connection)?;

    let mut vec = Vec::new();
    for ins in return_vec {
        let hash = ins.hash.clone();

        let _ = Object::from_database_object(ins)
            .map(|obj| vec.push(obj))
            .map_err(|e| {
                error!("[query - all]: Bad formated object [{:?}]: {:?}", hash, e);
            });
    }
    Ok(vec)
}

pub fn get(hash: GenericHash, connection: &PgConnection) -> QueryResult<Object> {
    let return_object: DatabaseObject = objects::table
        .find(&hash.to_vec())
        .get_result::<DatabaseObject>(connection)?;

    let obj = Object::from_database_object(return_object).unwrap();
    Ok(obj)
}

pub fn get_disposable(hash: &GenericHash) -> QueryResult<Object> {
    let pool = connection_pool::init_pool();
    let dbconn = pool.get().unwrap();
    let connection = dbconn.deref();

    let return_object: DatabaseObject = objects::table
        .find(hash.to_vec())
        .get_result::<DatabaseObject>(connection)?;

    let obj = Object::from_database_object(return_object).unwrap();
    Ok(obj)
}

pub fn insert(content: Content, connection: &PgConnection) -> QueryResult<Object> {
    let database_object = DatabaseObject::from_content(content);
    let return_object: DatabaseObject = diesel::insert_into(objects::table)
        .values(&database_object)
        .get_result(connection)?;

    let obj = Object::from_database_object(return_object).unwrap();
    Ok(obj)
}

pub fn delete(hash: GenericHash, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(objects::table.find(hash.to_vec())).execute(connection)
}
