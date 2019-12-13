use crypto::strings;
use database::schema::objects;
use primitives::object::{Content, Object};
use serde_json;
use serde_json::json;
use std::fmt;

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "objects"]
pub struct DatabaseObject {
    pub hash: Vec<u8>,
    pub content: serde_json::Value,
}

impl DatabaseObject {
    pub fn from_object(object: Object) -> DatabaseObject {
        let object_json = json!({
            "header": object.content.header,
            "body": object.content.body
        });

        DatabaseObject {
            hash: object.hash.to_vec(),
            content: object_json,
        }
    }

    pub fn from_content(content: Content) -> DatabaseObject {
        let content_json = json!(content);
        let hash = content.raw_hash().unwrap();

        DatabaseObject {
            hash: hash,
            content: content_json,
        }
    }
}

impl fmt::Display for DatabaseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "hash: {}, content: {}",
            strings::to_hex_string(&self.hash),
            self.content
        )
    }
}
