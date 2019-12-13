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
        let id = ins.hash.clone();
        let obj = match Object::from_database_object(ins) {
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
    let return_object: DatabaseObject = objects::table
        .find(&id.to_vec())
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

pub fn delete(id: GenericHash, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(objects::table.find(id.to_vec())).execute(connection)
}
