extern crate serde_json;

use serde_json::json;

#[allow(dead_code)]
pub fn relation_body(definition: &str, first_object: &str, second_object: &str) -> String {
    let json = json!({
        "definition": definition,
        "first_object": first_object,
        "second_object": second_object,
    });
    json.to_string()
}

#[allow(dead_code)]
pub fn json_relation_hash(json_response: serde_json::Value) -> String {
    json_response
        .get("hash")
        .unwrap()
        .as_str()
        .unwrap()
        .to_owned()
}

#[allow(dead_code)]
pub fn json_relation_base(json_response: serde_json::Value) -> serde_json::Value {
    json_response.get("relation_base").unwrap().to_owned()
}

#[allow(dead_code)]
pub fn json_relation_definition(json_response: serde_json::Value) -> serde_json::Value {
    json_relation_base(json_response)
        .get("definition")
        .unwrap()
        .to_owned()
}

#[allow(dead_code)]
pub fn json_relation_definition_hash(json_response: serde_json::Value) -> String {
    json_relation_definition(json_response)["hash"]
        .as_str()
        .unwrap()
        .to_owned()
}

#[allow(dead_code)]
pub fn json_relation_first_obj(json_response: serde_json::Value) -> serde_json::Value {
    json_relation_base(json_response)
        .get("first_object")
        .unwrap()
        .to_owned()
}

#[allow(dead_code)]
pub fn json_relation_first_obj_hash(json_response: serde_json::Value) -> String {
    json_relation_first_obj(json_response)["hash"]
        .as_str()
        .unwrap()
        .to_owned()
}

#[allow(dead_code)]
pub fn json_relation_second_obj(json_response: serde_json::Value) -> serde_json::Value {
    json_relation_base(json_response)
        .get("second_object")
        .unwrap()
        .to_owned()
}

#[allow(dead_code)]
pub fn json_relation_second_obj_hash(json_response: serde_json::Value) -> String {
    json_relation_second_obj(json_response)["hash"]
        .as_str()
        .unwrap()
        .to_owned()
}
