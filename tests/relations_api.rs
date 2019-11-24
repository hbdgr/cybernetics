extern crate rocket;
extern crate serde_json;

mod common;

use common::rocket_helpers::{create_test_object, rocket_client};
use rocket::http::{ContentType, Status};
use serde_json::json;

fn relation_body(object_definition_id: i64, first_object_id: i64, second_object_id: i64) -> String {
    let json = json!({
        "object_definition_id": object_definition_id,
        "first_object_id": first_object_id,
        "second_object_id": second_object_id,
    });
    json.to_string()
}

#[test]
fn create_relation() {
    let client = rocket_client();

    let obj_def_id = create_test_object("object def");
    let obj_first_id = create_test_object("first object");
    let obj_second_id = create_test_object("second object");

    let obj_body = relation_body(obj_def_id, obj_first_id, obj_second_id);

    let mut response = client
        .post("/relations")
        .body(obj_body)
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Created);

    let json_response: serde_json::Value =
        serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(
        json_response.get("object_definition_id").unwrap(),
        obj_def_id
    );
    assert_eq!(json_response.get("first_object_id").unwrap(), obj_first_id);
    assert_eq!(
        json_response.get("second_object_id").unwrap(),
        obj_second_id
    );
}
