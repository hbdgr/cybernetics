use crypto::hash::{raw_generic, GenericHash};
use database::schema::objects;
use primitives::object::Object;
use serde_json;
use serde_json::json;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "objects"]
pub struct QueryableObject {
    pub id: Vec<u8>,
    pub content: serde_json::Value,
}

impl QueryableObject {
    pub fn from_object(object: Object) -> QueryableObject {
        let object_json = json!({
            "header": object.header,
            "body": object.body
        });

        QueryableObject {
            id: object.id.to_vec(),
            content: object_json,
        }
    }

    pub fn from_insertable_object(id: GenericHash, object: InsertableObject) -> QueryableObject {
        QueryableObject {
            id: id.to_vec(),
            content: object.content,
        }
    }
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "objects"]
pub struct InsertableObject {
    pub content: serde_json::Value,
}

impl InsertableObject {
    pub fn from_object(object: Object) -> InsertableObject {
        let object_json = json!({
            "header": object.header,
            "body": object.body
        });

        InsertableObject {
            content: object_json,
        }
    }

    pub fn hash(&self) -> Result<Vec<u8>, ()> {
        let bytes = serde_json::to_vec(&self.content).unwrap();
        let hash = raw_generic(&bytes)?;
        Ok(hash)
    }
}
