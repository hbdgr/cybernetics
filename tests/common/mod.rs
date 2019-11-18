extern crate cybernetics;
extern crate rocket;

use self::cybernetics::server;
use common::rocket::local::Client;
use std::option;

pub fn rocket_client() -> Client {
    Client::new(server::router::create_routes()).expect("valid rocket instance")
}

// extract id from response body
pub fn response_body_id(res_body: option::Option<String>) -> i64 {
    let res_json: serde_json::Value = serde_json::from_str(&res_body.unwrap()).unwrap();
    res_json.get("id").unwrap().as_i64().unwrap()
}
