extern crate rocket;
extern crate serde_json;

mod common;

use common::rocket_helpers::{create_test_object, create_test_relation, rocket_client};
use rocket::http::{ContentType, Status};
use serde_json::json;

fn relation_body(definition: &str, first_object: &str, second_object: &str) -> String {
    let json = json!({
        "definition": definition,
        "first_object": first_object,
        "second_object": second_object,
    });
    json.to_string()
}

fn create_test_relation_body(postfix: &str) -> String {
    let def = create_test_object(&format!("object def {}", postfix));
    let obj_first = create_test_object(&format!("first object {}", postfix));
    let obj_second = create_test_object(&format!("second object {}", postfix));

    relation_body(&def, &obj_first, &obj_second)
}

fn json_relation_base(json_response: serde_json::Value) -> serde_json::Value {
    json_response.get("relation_base").unwrap().to_owned()
}

fn json_relation_definition(json_response: serde_json::Value) -> String {
    json_relation_base(json_response)["definition"]
        .as_str()
        .unwrap()
        .to_owned()
}

fn json_relation_first_obj(json_response: serde_json::Value) -> String {
    json_relation_base(json_response)["first_object"]
        .as_str()
        .unwrap()
        .to_owned()
}

fn json_relation_second_obj(json_response: serde_json::Value) -> String {
    json_relation_base(json_response)["second_object"]
        .as_str()
        .unwrap()
        .to_owned()
}

#[test]
fn create_relation() {
    let def_hash = create_test_object("object def");
    let obj_first_hash = create_test_object("first object");
    let obj_second_hash = create_test_object("second object");

    let rel_body = relation_body(&def_hash, &obj_first_hash, &obj_second_hash);

    let client = rocket_client();
    let mut response = client
        .post("/relations")
        .body(&rel_body)
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Created);

    let json_response: serde_json::Value =
        serde_json::from_str(&response.body_string().unwrap()).unwrap();

    assert_eq!(json_relation_definition(json_response.clone()), def_hash);
    assert_eq!(
        json_relation_first_obj(json_response.clone()),
        obj_first_hash
    );
    assert_eq!(json_relation_second_obj(json_response), obj_second_hash);
}

#[test]
fn get_relation_by_hash() {
    let rel_body = create_test_relation_body("1");
    let relation_hash = create_test_relation(&rel_body);

    println!("created {}", relation_hash);
    let client = rocket_client();

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
    create_test_relation(&create_test_relation_body("aa"));
    create_test_relation(&create_test_relation_body("bb"));

    let client = rocket_client();
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
        json_relation_base(response_array[0].clone())
            .get("definition")
            .unwrap()
            .is_string(),
        "relation definition should be a hex string"
    );
}

#[test]
fn put_relation() {
    let def_hash = create_test_object("put definition");
    let obj_first_hash = create_test_object("first put object");
    let obj_second_hash = create_test_object("second put object");

    let mut rel_body = relation_body(&def_hash, &obj_first_hash, &obj_second_hash);
    let relation_hash = create_test_relation(&rel_body);

    let new_def_hash = create_test_object("new_definition");
    rel_body = relation_body(&new_def_hash, &obj_first_hash, &obj_second_hash);

    let client = rocket_client();
    let mut response = client
        .put(format!("/relations/{}", relation_hash))
        .body(&rel_body)
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response.status(), Status::Created);

    // conflict for existing relation
    let response_conflict = client
        .put(format!("/relations/{}", relation_hash))
        .body(&rel_body)
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(response_conflict.status(), Status::Conflict);

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

    assert_ne!(json_relation_definition(json_response.clone()), def_hash);

    assert_eq!(
        json_relation_definition(json_response.clone()),
        new_def_hash
    );
    assert_eq!(
        json_relation_first_obj(json_response.clone()),
        obj_first_hash
    );
    assert_eq!(json_relation_second_obj(json_response), obj_second_hash);
}

#[test]
fn delete_relation() {
    let rel_body = create_test_relation_body("D");
    let relation_hash = create_test_relation(&rel_body);

    let client = rocket_client();
    let response = client
        .delete(format!("/relations/{}", relation_hash))
        .dispatch();

    assert_eq!(response.status(), Status::NoContent);

    let response = client
        .get(format!("/relations/{}", relation_hash))
        .dispatch();
    assert_eq!(response.status(), Status::NotFound);
}
