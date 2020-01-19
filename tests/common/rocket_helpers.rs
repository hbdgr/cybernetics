extern crate cybernetics;
extern crate rocket;

use self::cybernetics::database;
use self::cybernetics::server;
use super::header::ObjectType;
use common::object_helpers;
use common::relation_helpers;
use common::rocket_helpers::rocket::http::{ContentType, Status};
use common::rocket_helpers::rocket::local::Client;
use std::option;

#[allow(dead_code)]
pub fn rocket_client() -> Client {
    let db_pool = database::connection_pool::init_pool();
    Client::new(server::router::create_routes(db_pool)).expect("valid rocket instance")
}

// extract id from response body
#[allow(dead_code)]
fn response_body_hash(res_body: option::Option<String>) -> String {
    let res_json: serde_json::Value = serde_json::from_str(&res_body.unwrap()).unwrap();
    res_json.get("hash").unwrap().as_str().unwrap().to_string()
}

// create test primary_element in database by POST method, with given body and return its id
#[allow(dead_code)]
pub fn create_test_element(body_str: &str) -> String {
    let response_body =
        create_test_object_expect_status(ObjectType::PrimaryElement, body_str, Status::Created);
    response_body_hash(response_body)
}

// create test relation_definition in database by POST method, with given body and return its id
// directed: relation can be directed or not
#[allow(dead_code)]
pub fn create_test_relation_def(directed: bool, body_str: &str) -> String {
    let response_body = create_test_object_expect_status(
        ObjectType::RelationDefinition { directed: directed },
        body_str,
        Status::Created,
    );
    response_body_hash(response_body)
}

// create test object, and expect given status
#[allow(dead_code)]
pub fn create_test_object_expect_status(
    object_type: ObjectType,
    body_str: &str,
    status: Status,
) -> std::option::Option<String> {
    let full_body = object_helpers::test_content_json(object_type, body_str).to_string();

    let client = rocket_client();
    let mut response = client
        .post("/objects")
        .body(full_body)
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), status);
    response.body_string()
}

#[allow(dead_code)]
pub fn create_test_relation(body_str: &str) -> String {
    let response_body = create_test_relation_expect_status(body_str, Status::Created);
    response_body_hash(response_body)
}

// create test relation, and expect given status
#[allow(dead_code)]
pub fn create_test_relation_expect_status(
    body_str: &str,
    status: Status,
) -> std::option::Option<String> {
    let client = rocket_client();
    let mut response = client
        .post("/relations")
        .body(body_str)
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), status);
    response.body_string()
}

#[allow(dead_code)]
pub fn create_test_relation_body(directed: bool, postfix: &str) -> String {
    let def = create_test_relation_def(directed, &format!("object def {}", postfix));
    let obj_first = create_test_element(&format!("first object {}", postfix));
    let obj_second = create_test_element(&format!("second object {}", postfix));

    relation_helpers::relation_body(&def, &obj_first, &obj_second)
}
