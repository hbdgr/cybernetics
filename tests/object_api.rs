extern crate cybernetics;
extern crate rocket;
extern crate serde_json;

use cybernetics::server;
use rocket::http::{ContentType, Status};
use rocket::local::Client;
// use serde_json::json;

static OBJ_BODY: &str = r#"{"content":{"header":"header","body":"test_objcet"}}"#;

fn rocket_client() -> Client {
    Client::new(server::router::create_routes()).expect("valid rocket instance")
}

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
    client
        .post("/objects")
        .body(OBJ_BODY)
        .header(ContentType::JSON)
        .dispatch();

    let mut response = client.get("/objects/1").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let json_response: serde_json::Value =
        serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(json_response.get("header").unwrap(), "header");
    assert_eq!(json_response.get("body").unwrap(), "test_objcet");
}

#[test]
fn delete_objcet() {
    let client = rocket_client();
    let mut post_response = client
        .post("/objects")
        .body(OBJ_BODY)
        .header(ContentType::JSON)
        .dispatch();

    let post_json_response: serde_json::Value =
        serde_json::from_str(&post_response.body_string().unwrap()).unwrap();
    let created_obj_id = post_json_response.get("id").unwrap().as_i64().unwrap();

    let response = client
        .delete(format!("/objects/{}", created_obj_id))
        .dispatch();
    assert_eq!(response.status(), Status::NoContent);
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
