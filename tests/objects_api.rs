extern crate rocket;
extern crate serde_json;

mod common;

use common::rocket_helpers::{create_test_object, rocket_client};
use rocket::http::{ContentType, Status};

static OBJ_BODY: &str = r#"{"content":{"header":"header","body":"test_objcet"}}"#;

#[test]
fn create_object() {
    let client = rocket_client();
    let mut response = client
        .post("/objects")
        .body(OBJ_BODY)
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Created);

    let json_response: serde_json::Value =
        serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(json_response.get("header").unwrap(), "header");
    assert_eq!(json_response.get("body").unwrap(), "test_objcet");
}

#[test]
fn get_objcet() {
    let body_str = "obj_to_get";
    let created_obj_id = create_test_object(body_str);

    let client = rocket_client();
    let mut response = client
        .get(format!("/objects/{}", created_obj_id))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    let json_response: serde_json::Value =
        serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(json_response.get("header").unwrap(), "header");
    assert_eq!(json_response.get("body").unwrap(), body_str);
}

#[test]
fn get_all() {
    create_test_object("obj1");
    create_test_object("obj2");

    let client = rocket_client();
    let mut response = client.get("/objects").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let json_response: serde_json::Value =
        serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(json_response.is_array(), true);
    assert!(json_response.as_array().unwrap().len() > 0);
}

#[test]
fn delete_objcet() {
    let body_str = "obj_to_delete";
    let created_obj_id = create_test_object(body_str);

    let client = rocket_client();
    let response = client
        .delete(format!("/objects/{}", created_obj_id))
        .dispatch();
    assert_eq!(response.status(), Status::NoContent);
}
