use database::object::{InsertableObject, Object, QueryableObject};
use database::schema::objects;
use diesel;
use diesel::prelude::*;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Object>> {
    let queryable_vec = objects::table.load::<QueryableObject>(&*connection)?;

    let mut vec = Vec::new();
    for ins in queryable_vec {
        let id = ins.id;
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

pub fn get(id: i64, connection: &PgConnection) -> QueryResult<Object> {
    let queryable = objects::table
        .find(id)
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

pub fn update(id: i64, object: Object, connection: &PgConnection) -> QueryResult<Object> {
    let insertable = InsertableObject::from_object(object);

    let queryable = diesel::update(objects::table.find(id))
        .set(&insertable)
        .get_result(connection)?;

    let obj = Object::from_queryable_object(queryable).unwrap();
    Ok(obj)
}

pub fn delete(id: i64, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(objects::table.find(id)).execute(connection)
}
