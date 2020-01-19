extern crate serde_json;

use super::header::{Header, ObjectType};
use super::object::{Content, Object};
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

#[allow(dead_code)]
pub fn test_object(object_type: ObjectType, body: &str) -> Object {
    let content = test_content(object_type, body);

    Object::from_content(content).unwrap()
}

#[allow(dead_code)]
pub fn test_primary_element(body: &str) -> Object {
    test_object(ObjectType::PrimaryElement, body)
}

#[allow(dead_code)]
pub fn test_relation_definition(directed: bool, body: &str) -> Object {
    test_object(ObjectType::RelationDefinition { directed: directed }, body)
}
