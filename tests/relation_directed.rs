extern crate rocket;
extern crate serde_json;

mod common;

use common::relation_helpers;
use common::rocket_helpers;
use rocket::http::{ContentType, Status};

#[test]
fn create_directed_relation() {
    let hash_ordered = "a52c893e2101c248c6035c27552687b126a9bde563ac6109e4dc2ac8bb9ca89d";

    let def_hash = rocket_helpers::create_test_relation_def(true, "directed relation definition");
    let obj_first_hash = rocket_helpers::create_test_element("dir first object");
    let obj_second_hash = rocket_helpers::create_test_element("dir second object");

    let rel_body = relation_helpers::relation_body(&def_hash, &obj_first_hash, &obj_second_hash);

    let client = rocket_helpers::rocket_client();
    let mut response = client
        .post("/relations")
        .body(&rel_body)
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Created);

    let json_response: serde_json::Value =
        serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(
        hash_ordered,
        relation_helpers::json_relation_hash(json_response),
    );

    // objects order switched
    let rel_body_unordered =
        relation_helpers::relation_body(&def_hash, &obj_second_hash, &obj_first_hash);

    let mut response_unordered = client
        .post("/relations")
        .body(&rel_body_unordered)
        .header(ContentType::JSON)
        .dispatch();

    // there should be no conflict, switched objects creates new, different relation
    assert_eq!(response_unordered.status(), Status::Created);

    let json_response_unordered: serde_json::Value =
        serde_json::from_str(&response_unordered.body_string().unwrap()).unwrap();

    // not equal
    assert_ne!(
        hash_ordered,
        relation_helpers::json_relation_hash(json_response_unordered),
    );
}
