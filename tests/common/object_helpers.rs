extern crate serde_json;

use super::header::{Header, ObjectType};
use super::object::Content;
use serde_json::json;

pub fn test_content(object_type: ObjectType, body: &str) -> Content {
    Content {
        header: Header {
            object_type: object_type,
        },
        body: body.to_string(),
    }
}

#[allow(dead_code)]
pub fn test_content_json(object_type: ObjectType, body: &str) -> serde_json::Value {
    let object = test_content(object_type, body);
    json!(object)
}
