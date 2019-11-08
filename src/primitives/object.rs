use database::object::{InsertableObject, QueryableObject};

#[derive(Serialize, Deserialize, Debug)]
pub struct Object {
    #[serde(skip_deserializing)]
    pub id: i64,
    pub header: String,
    pub body: String,
}

impl Object {
    pub fn from_insertable_object(insertable_object: InsertableObject) -> Result<Object, String> {
        let obj: Object = match serde_json::from_value(insertable_object.content) {
            Ok(obj) => obj,
            Err(err) => return Err(err.to_string()),
        };
        Ok(obj)
    }

    pub fn from_queryable_object(queryable_object: QueryableObject) -> Result<Object, String> {
        let mut obj: Object = match serde_json::from_value(queryable_object.content) {
            Ok(obj) => obj,
            Err(err) => return Err(err.to_string()),
        };

        // id was skipped with serde attribute
        obj.id = queryable_object.id;
        Ok(obj)
    }
}
