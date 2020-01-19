extern crate rocket;
extern crate serde_json;

mod common;

use common::relation_helpers;
use common::rocket_helpers;
use rocket::http::{ContentType, Status};

#[test]
fn create_relation() {
    let def_hash = rocket_helpers::create_test_relation_def(false, "object def");
    let obj_first_hash = rocket_helpers::create_test_element("first object");
    let obj_second_hash = rocket_helpers::create_test_element("second object");

    let rel_body = relation_helpers::relation_body(&def_hash, &obj_first_hash, &obj_second_hash);
    println!("rol_body: {}", rel_body);

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
        relation_helpers::json_relation_definition_hash(json_response.clone()),
        def_hash
    );
    assert_eq!(
        relation_helpers::json_relation_first_obj_hash(json_response.clone()),
        obj_first_hash
    );
    assert_eq!(
        relation_helpers::json_relation_second_obj_hash(json_response),
        obj_second_hash
    );
}

#[test]
fn create_duplication() {
    let rel_body = rocket_helpers::create_test_relation_body(false, "create_duplication");
    rocket_helpers::create_test_relation(&rel_body);
    rocket_helpers::create_test_relation_expect_status(&rel_body, Status::Conflict);
}

#[test]
fn get_relation_by_hash() {
    let rel_body = rocket_helpers::create_test_relation_body(false, "1");
    let relation_hash = rocket_helpers::create_test_relation(&rel_body);

    println!("created {}", relation_hash);
    let client = rocket_helpers::rocket_client();

    let mut response = client
        .get(format!("/relations/{}", relation_hash))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let json_response: serde_json::Value =
        serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(json_response.get("hash").unwrap(), &relation_hash);
}

#[test]
fn get_all_relations() {
    rocket_helpers::create_test_relation(&rocket_helpers::create_test_relation_body(false, "aa"));
    rocket_helpers::create_test_relation(&rocket_helpers::create_test_relation_body(false, "bb"));

    let client = rocket_helpers::rocket_client();
    let mut response = client.get("/relations").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let json_response: serde_json::Value =
        serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(json_response.is_array(), true);

    let response_array = json_response.as_array().unwrap();
    assert!(
        response_array.len() > 0,
        "lenght of array should be greater than 0"
    );

    assert!(
        relation_helpers::json_relation_definition(response_array[0].clone())
            .get("hash")
            .unwrap()
            .is_string(),
        "relation definition hash should be a hex string"
    );
}

#[test]
fn put_relation() {
    let def_hash = rocket_helpers::create_test_relation_def(false, "put definition");
    let obj_first_hash = rocket_helpers::create_test_element("first put object");
    let obj_second_hash = rocket_helpers::create_test_element("second put object");

    let mut rel_body =
        relation_helpers::relation_body(&def_hash, &obj_first_hash, &obj_second_hash);
    let relation_hash = rocket_helpers::create_test_relation(&rel_body);

    let new_def_hash = rocket_helpers::create_test_element("new_definition");
    rel_body = relation_helpers::relation_body(&new_def_hash, &obj_first_hash, &obj_second_hash);

    let client = rocket_helpers::rocket_client();
    let mut response = client
        .put(format!("/relations/{}", relation_hash))
        .body(&rel_body)
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Created);

    // old relation should be deleted
    let old_rel_response = client
        .get(format!("/relations/{}", relation_hash))
        .dispatch();
    assert_eq!(old_rel_response.status(), Status::NotFound);

    let json_response: serde_json::Value =
        serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert!(
        relation_hash != json_response["hash"].as_str().unwrap(),
        "new hash should be different"
    );

    assert_ne!(
        relation_helpers::json_relation_definition_hash(json_response.clone()),
        def_hash
    );

    assert_eq!(
        relation_helpers::json_relation_definition_hash(json_response.clone()),
        new_def_hash
    );
    assert_eq!(
        relation_helpers::json_relation_first_obj_hash(json_response.clone()),
        obj_first_hash
    );
    assert_eq!(
        relation_helpers::json_relation_second_obj_hash(json_response),
        obj_second_hash
    );
}

#[test]
fn put_duplication() {
    let rel_body = rocket_helpers::create_test_relation_body(false, "put_duplication");
    let relation_hash = rocket_helpers::create_test_relation(&rel_body);

    let client = rocket_helpers::rocket_client();

    // conflict for existing relation
    let response_conflict = client
        .put(format!("/relations/{}", relation_hash))
        .body(&rel_body)
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response_conflict.status(), Status::Conflict);
}

#[test]
fn delete_relation() {
    let rel_body = rocket_helpers::create_test_relation_body(false, "D");
    let relation_hash = rocket_helpers::create_test_relation(&rel_body);

    let client = rocket_helpers::rocket_client();
    let response = client
        .delete(format!("/relations/{}", relation_hash))
        .dispatch();

    assert_eq!(response.status(), Status::NoContent);

    let response = client
        .get(format!("/relations/{}", relation_hash))
        .dispatch();
    assert_eq!(response.status(), Status::NotFound);
}
