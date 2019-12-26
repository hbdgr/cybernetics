use crypto::hash::GenericHash;
use primitives::object::Object;

use database::object::DatabaseObject;
use database::schema::objects;
use diesel;
use diesel::prelude::*;

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

pub fn insert(object: DatabaseObject, connection: &PgConnection) -> QueryResult<Object> {
    let return_object: DatabaseObject = diesel::insert_into(objects::table)
        .values(&object)
        .get_result(connection)?;

    let obj = Object::from_database_object(return_object).unwrap();
    Ok(obj)
}

pub fn delete(hash: GenericHash, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(objects::table.find(hash.to_vec())).execute(connection)
}
