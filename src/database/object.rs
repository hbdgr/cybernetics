use database::schema::objects;


#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "objects"]
pub struct Object {
	pub id: i32,
	pub content: String,
}

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "objects"]
pub struct InsertableObject {
	pub content: String,
}

impl InsertableObject {
	pub fn from_object(object: Object) -> InsertableObject {
		InsertableObject {
			content: object.content,
		}
	}
}
