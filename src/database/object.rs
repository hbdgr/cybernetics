use database::schema::objects;
use serde_json::json;

#[derive(Serialize, Deserialize)]
// #[derive(Queryable, AsChangeset, Serialize, Deserialize)]
// #[table_name = "objects"]
pub struct Object {
	pub id: i32,
	pub header: String,
	pub body:   String,
}

impl Object {
    pub fn from_insertable_object(insertable_object: InsertableObject) -> Object {
        let obj: Object = serde_json::from_value(insertable_object.object)?;

        obj
    }
}

#[derive(Insertable, Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "objects"]
pub struct InsertableObject {
    pub object: serde_json::Value,
}

impl InsertableObject {
	pub fn from_object(object: Object) -> InsertableObject {
        let object_json = json!({
            "header": object.header,
            "body": object.body
        });

		InsertableObject {
			object: object_json,
		}
	}
}
