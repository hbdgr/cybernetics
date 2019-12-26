extern crate cybernetics;
extern crate rocket;
extern crate serde_json;

mod common;

use common::rocket_helpers::{create_test_object, create_test_object_expect_status, rocket_client};
use rocket::http::{ContentType, Status};
use serde_json::json;

static OBJ_BODY: &str = r#"{"header":"header","body":"test_object"}"#;

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

    assert_eq!(
        json_response.get("content").unwrap(),
        &json!({"header":"header","body":"test_object"})
    );

    assert_eq!(
        json_response["hash"].as_str(),
        Some("d3d5a23242a3a632ac3cf93763c24ec6dbc2a2aef436cdc582c2a931781a6063")
    );
}

#[test]
fn create_duplication() {
    let body_str = "duplicate";
    create_test_object(body_str);
    create_test_object_expect_status(body_str, Status::Conflict);
}

#[test]
fn get_object() {
    let created_obj_hash = create_test_object("obj_to_get");

    let client = rocket_client();
    let mut response = client
        .get(format!("/objects/{}", created_obj_hash))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    let json_response: serde_json::Value =
        serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(
        json_response.get("content").unwrap(),
        &json!({"header":"header","body":"obj_to_get"})
    );

    assert_eq!(
        json_response["hash"].as_str(),
        Some("2725827c44d3b35fde93b556b4649535a416f3d9a54c567eba4542e724994980")
    );
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
    assert!(
        json_response.as_array().unwrap().len() > 0,
        "lenght of array should be greater than 0"
    );
}

#[test]
fn put_object() {
    let created_obj_hash = create_test_object("before_put");
    let new_body = r#"{"header":"header","body":"new_better.."}"#;

    let client = rocket_client();
    let mut response = client
        .put(format!("/objects/{}", created_obj_hash))
        .body(&new_body)
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Created);

    // conflict for existing object
    let response_conflict = client
        .put(format!("/objects/{}", created_obj_hash))
        .body(&new_body)
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response_conflict.status(), Status::Conflict);

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
        &json!({"header":"header","body":"new_better.."})
    );

    assert_eq!(
        json_response["hash"].as_str(),
        Some("8774b510a7950450834c6ddfce25ffd2829e1d1ce09bc0842a53b2502843f1d5")
    );
}

#[test]
fn put_duplicated() {
    let body_str = "put_duplicated";
    let created_obj_hash = create_test_object(body_str);
    let mut new_same_body = r#"{"header":"header","body":""#.to_string();
    new_same_body.push_str(body_str);
    new_same_body.push_str(r#""}"#);

    let client = rocket_client();
    let response = client
        .put(format!("/objects/{}", created_obj_hash))
        .body(&new_same_body)
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Conflict);
}

#[test]
fn delete_object() {
    let body_str = "obj_to_delete";
    let created_obj_hash = create_test_object(body_str);

    let client = rocket_client();
    let response = client
        .delete(format!("/objects/{}", created_obj_hash))
        .dispatch();
    assert_eq!(response.status(), Status::NoContent);

    let response = client
        .get(format!("/objects/{}", created_obj_hash))
        .dispatch();
    assert_eq!(response.status(), Status::NotFound);
}
