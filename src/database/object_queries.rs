use diesel;
use diesel::prelude::*;
use database::schema::objects;
use database::object::{Object, InsertableObject};


pub fn all(connection: &PgConnection) -> QueryResult<Vec<Object>> {
	objects::table.load::<Object>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Object> {
	objects::table.find(id).get_result::<Object>(connection)
}

// TODO avoid duplications
pub fn insert(object: InsertableObject, connection: &PgConnection) -> QueryResult<Object> {
	diesel::insert_into(objects::table)
		.values(&object)
		.get_result(connection)
}

pub fn update(id: i32, object: Object, connection: &PgConnection) -> QueryResult<Object> {
	diesel::update(objects::table.find(id))
		.set(&object)
		.get_result(connection)
}
pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
	diesel::delete(objects::table.find(id))
		.execute(connection)
}
