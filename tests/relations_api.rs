extern crate rocket;
extern crate serde_json;

mod common;

use common::rocket_helpers::{create_test_object, create_test_relation, rocket_client};
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

fn create_test_relation_body(postfix: &str) -> String {
    let obj_def_id = create_test_object(&format!("object def {}", postfix));
    let obj_first_id = create_test_object(&format!("first object {}", postfix));
    let obj_second_id = create_test_object(&format!("second object {}", postfix));

    relation_body(obj_def_id, obj_first_id, obj_second_id)
}

#[test]
fn create_relation() {
    let obj_def_id = create_test_object("object def");
    let obj_first_id = create_test_object("first object");
    let obj_second_id = create_test_object("second object");

    let obj_body = relation_body(obj_def_id, obj_first_id, obj_second_id);

    let client = rocket_client();
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

#[test]
fn get_relation_by_id() {
    let obj_body = create_test_relation_body("1");
    let relation_id = create_test_relation(&obj_body);

    let client = rocket_client();

    let mut response = client.get(format!("/relations/{}", relation_id)).dispatch();

    assert_eq!(response.status(), Status::Ok);

    let json_response: serde_json::Value =
        serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(json_response.get("id").unwrap(), relation_id);
}

#[test]
fn get_all_relations() {
    create_test_relation(&create_test_relation_body("aa"));
    create_test_relation(&create_test_relation_body("bb"));

    let client = rocket_client();
    let mut response = client.get("/relations").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let json_response: serde_json::Value =
        serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(json_response.is_array(), true);

    let response_array = json_response.as_array().unwrap();
    assert!(response_array.len() > 0);

    assert!(response_array[0]
        .get("object_definition_id")
        .unwrap()
        .is_i64());
}

#[test]
fn delete_relation() {
    let obj_body = create_test_relation_body("D");
    let relation_id = create_test_relation(&obj_body);

    let client = rocket_client();
    let response = client
        .delete(format!("/relations/{}", relation_id))
        .dispatch();

    assert_eq!(response.status(), Status::NoContent);
}
