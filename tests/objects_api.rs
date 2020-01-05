extern crate cybernetics;
extern crate rocket;
extern crate serde_json;

mod common;

use common::object_helpers;
use common::rocket_helpers;
use cybernetics::primitives::header::ObjectType;
use rocket::http::{ContentType, Status};
use serde_json::json;

#[test]
fn create_object() {
    let obj_json = object_helpers::test_content_json(ObjectType::PrimaryElement, "test_object");

    let client = rocket_helpers::rocket_client();
    let mut response = client
        .post("/objects")
        .body(obj_json.to_string())
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Created);

    let json_response: serde_json::Value =
        serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(
        json_response.get("content").unwrap(),
        &json!({"header": { "object_type": "PrimaryElement"},"body":"test_object"})
    );

    assert_eq!(
        json_response["hash"].as_str(),
        Some("f1dee37be385017d470584765ae9dd577a4a189b4f5c1320a912d71fd2ec92b5")
    );
}

#[test]
fn create_duplication() {
    let body_str = "duplicate";
    rocket_helpers::create_test_element(body_str);
    rocket_helpers::create_test_object_expect_status(
        ObjectType::PrimaryElement,
        body_str,
        Status::Conflict,
    );
}

#[test]
fn get_object() {
    let created_obj_hash = rocket_helpers::create_test_element("obj_to_get");

    let client = rocket_helpers::rocket_client();
    let mut response = client
        .get(format!("/objects/{}", created_obj_hash))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    let json_response: serde_json::Value =
        serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(
        json_response.get("content").unwrap(),
        &json!({"header": { "object_type": "PrimaryElement"},"body":"obj_to_get"})
    );

    assert_eq!(
        json_response["hash"].as_str(),
        Some("c2c3061b44f977c4cf1cf690806b0bb7cb3b1f0233a0f27343281a93c7486cb2")
    );
}

#[test]
fn get_all() {
    rocket_helpers::create_test_element("obj1");
    rocket_helpers::create_test_element("obj2");

    let client = rocket_helpers::rocket_client();
    let mut response = client.get("/objects").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let json_response: serde_json::Value =
        serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(json_response.is_array(), true);
    assert!(
        json_response.as_array().unwrap().len() > 0,
        "lenght of array should be greater than 0"
    );
}

#[test]
fn put_object() {
    let created_obj_hash = rocket_helpers::create_test_element("before_put");
    let new_body_json =
        object_helpers::test_content_json(ObjectType::PrimaryElement, "new_better..");

    let client = rocket_helpers::rocket_client();
    let mut response = client
        .put(format!("/objects/{}", created_obj_hash))
        .body(&new_body_json.to_string())
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Created);

    // old object should be deleted
    let old_obj_response = client
        .get(format!("/objects/{}", created_obj_hash))
        .dispatch();
    assert_eq!(old_obj_response.status(), Status::NotFound);

    let json_response: serde_json::Value =
        serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert!(
        created_obj_hash != json_response["hash"].as_str().unwrap(),
        "new hash should be different"
    );

    assert_eq!(
        json_response.get("content").unwrap(),
        &json!({"header": { "object_type": "PrimaryElement"},"body":"new_better.."})
    );

    assert_eq!(
        json_response["hash"].as_str(),
        Some("de4be1db48876db43a0127de88d1e6fab4dbe52689eff4ff450a1942a444595b"),
    );
}

#[test]
fn put_duplicated() {
    let body_str = "put_duplicated";
    let created_obj_hash = rocket_helpers::create_test_element(body_str);
    let new_same_body_json =
        object_helpers::test_content_json(ObjectType::PrimaryElement, body_str);

    let client = rocket_helpers::rocket_client();

    // conflict for existing object
    let response_conflict = client
        .put(format!("/objects/{}", created_obj_hash))
        .body(&new_same_body_json.to_string())
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response_conflict.status(), Status::Conflict);
}

#[test]
fn delete_object() {
    let body_str = "obj_to_delete";
    let created_obj_hash = rocket_helpers::create_test_element(body_str);

    let client = rocket_helpers::rocket_client();
    let response = client
        .delete(format!("/objects/{}", created_obj_hash))
        .dispatch();
    assert_eq!(response.status(), Status::NoContent);

    let response = client
        .get(format!("/objects/{}", created_obj_hash))
        .dispatch();
    assert_eq!(response.status(), Status::NotFound);
}
