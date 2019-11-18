extern crate cybernetics;
extern crate rocket;
extern crate serde_json;

mod common;

use common::{response_body_id, rocket_client};
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
    let client = rocket_client();
    let mut post_response = client
        .post("/objects")
        .body(OBJ_BODY)
        .header(ContentType::JSON)
        .dispatch();

    let created_obj_id = response_body_id(post_response.body_string());

    let mut response = client
        .get(format!("/objects/{}", created_obj_id))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    let json_response: serde_json::Value =
        serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(json_response.get("header").unwrap(), "header");
    assert_eq!(json_response.get("body").unwrap(), "test_objcet");
}

#[test]
fn get_all() {
    let client = rocket_client();
    client
        .post("/objects")
        .body(OBJ_BODY)
        .header(ContentType::JSON)
        .dispatch();

    let mut response = client.get("/objects").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let json_response: serde_json::Value =
        serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(json_response.is_array(), true);
    assert!(json_response.as_array().unwrap().len() > 0);
}

#[test]
fn delete_objcet() {
    let client = rocket_client();
    let mut post_response = client
        .post("/objects")
        .body(OBJ_BODY)
        .header(ContentType::JSON)
        .dispatch();

    let created_obj_id = response_body_id(post_response.body_string());

    let response = client
        .delete(format!("/objects/{}", created_obj_id))
        .dispatch();
    assert_eq!(response.status(), Status::NoContent);
}
