extern crate cybernetics;
extern crate rocket;

use self::cybernetics::server;
use common::rocket_helpers::rocket::http::{ContentType, Status};
use common::rocket_helpers::rocket::local::Client;
use std::option;

#[allow(dead_code)]
pub fn rocket_client() -> Client {
    Client::new(server::router::create_routes()).expect("valid rocket instance")
}

// extract id from response body
#[allow(dead_code)]
fn response_body_id(res_body: option::Option<String>) -> i64 {
    let res_json: serde_json::Value = serde_json::from_str(&res_body.unwrap()).unwrap();
    res_json.get("id").unwrap().as_i64().unwrap()
}

// create test object in database, with given body and return its id
#[allow(dead_code)]
pub fn create_test_object(body_str: &str) -> i64 {
    let mut full_body = r#"{"content":{"header":"header","body":""#.to_string();
    full_body.push_str(body_str);
    full_body.push_str(r#""}}"#);

    let client = rocket_client();
    let mut response = client
        .post("/objects")
        .body(full_body)
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Created);
    response_body_id(response.body_string())
}
