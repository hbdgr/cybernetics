extern crate cybernetics;
extern crate serde_json;

use cybernetics::crypto::{hash, msg_block, person, string};
use cybernetics::database::object::InsertableObject;
use serde_json::json;

#[test]
fn person_singature() {
    let first_struct = msg_block::build_msg_block("author", "example message");
    assert_eq!(
        "example message, author: author",
        format!("{}", first_struct)
    );

    let example_pass = "very_hard_to_break_password";
    let example_person = person::create_person("Amelino", example_pass);

    let msg_to_sign = "example message to sign";

    let signature = example_person.sign_msg(msg_to_sign, example_pass).unwrap();
    example_person.verify(msg_to_sign, &signature).unwrap();
}

#[test]
fn generic_hash() {
    let bytes = "abcd abcd".to_owned();
    let hash = hash::raw_generic(bytes.as_bytes()).unwrap();

    assert_eq!(
        "8d22328f614a98ea2e8555fc122eb0f79e04c0f1e9050952c4f59dca5da3040e",
        string::to_hex_string(&hash)
    );

    let bytes = "!@#@$@#$!@#!@cecdjbq12".to_owned();
    let hash = hash::raw_generic(bytes.as_bytes()).unwrap();

    assert_eq!(
        "cc2e8f56b943427eaf3be124825385c750ecc9d00dfeae1ea98bc1b4b81da346",
        string::to_hex_string(&hash)
    );
}

#[test]
fn object_hash() {
    let expected = "5f0468006dbd8c7b68c6c142e5e4c5ac2dad81c5f2e02b8fbe93a09814e2ff10";

    let obj = InsertableObject {
        content: json!({
            "_id": "5de90366f4b446b4a99daf7e",
            "guid": "653c0c70-b7e6-4f99-895e-e8e8f503979a",
            "balance": "₿2,231.27",
            "age": 36,
            "eyeColor": "brown",
            "name": "Nellie Lamb",
            "gender": "female",
            "tags": [ "labore", "excepteur", "consequat", "tempor" ],
            "friends": [ { "id": 0, "name": "Fuentes Downs" }, { "id": 1, "name": "Madeleine Mcclure" } ]
        }),
    };

    assert_eq!(expected, string::to_hex_string(&obj.hash().unwrap()));

    // same json, but different order
    let obj2 = InsertableObject {
        content: json!({
            "tags": [ "labore", "excepteur", "consequat", "tempor" ],
            "guid": "653c0c70-b7e6-4f99-895e-e8e8f503979a",
            "eyeColor": "brown",
            "_id": "5de90366f4b446b4a99daf7e",
            "age": 36,
            "name": "Nellie Lamb",
            "gender": "female",
            "friends": [ { "name": "Fuentes Downs", "id": 0 }, { "id": 1, "name": "Madeleine Mcclure" } ],
            "balance": "₿2,231.27"
        }),
    };

    assert_eq!(expected, string::to_hex_string(&obj2.hash().unwrap()));
}
