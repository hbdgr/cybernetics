extern crate cybernetics;
extern crate serde_json;

use cybernetics::crypto::{hash, msg_block, person, strings};
use cybernetics::database::object::DatabaseObject;
use cybernetics::primitives::object::Content;
use cybernetics::primitives::relation::RelationBase;
use hash::GenericHash;
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
        strings::to_hex_string(&hash)
    );

    let bytes = "!@#@$@#$!@#!@cecdjbq12".to_owned();
    let hash = hash::raw_generic(bytes.as_bytes()).unwrap();

    assert_eq!(
        "cc2e8f56b943427eaf3be124825385c750ecc9d00dfeae1ea98bc1b4b81da346",
        strings::to_hex_string(&hash)
    );
}

#[test]
fn json_hash() {
    let expected = "5f0468006dbd8c7b68c6c142e5e4c5ac2dad81c5f2e02b8fbe93a09814e2ff10";

    let json = json!({
            "_id": "5de90366f4b446b4a99daf7e",
            "guid": "653c0c70-b7e6-4f99-895e-e8e8f503979a",
            "balance": "₿2,231.27",
            "age": 36,
            "eyeColor": "brown",
            "name": "Nellie Lamb",
            "gender": "female",
            "tags": [ "labore", "excepteur", "consequat", "tempor" ],
            "friends": [ { "id": 0, "name": "Fuentes Downs" }, { "id": 1, "name": "Madeleine Mcclure" } ]
    });
    let json_bytes = serde_json::to_vec(&json).unwrap();
    let hash = hash::raw_generic(&json_bytes).unwrap();

    assert_eq!(expected, strings::to_hex_string(&hash));

    // same json, but different order
    let json2 = json!({
            "tags": [ "labore", "excepteur", "consequat", "tempor" ],
            "guid": "653c0c70-b7e6-4f99-895e-e8e8f503979a",
            "eyeColor": "brown",
            "_id": "5de90366f4b446b4a99daf7e",
            "age": 36,
            "name": "Nellie Lamb",
            "gender": "female",
            "friends": [ { "name": "Fuentes Downs", "id": 0 }, { "id": 1, "name": "Madeleine Mcclure" } ],
            "balance": "₿2,231.27"
    });
    let json2_bytes = serde_json::to_vec(&json2).unwrap();
    let hash2 = hash::raw_generic(&json2_bytes).unwrap();

    assert_eq!(expected, strings::to_hex_string(&hash2));
}

#[test]
fn object_hash_conversion() {
    let expected = "50d75e3df52981e4053bc5030a3bfd5a4de5ab994e0fbe6215601091b144c02d";

    let ctx = Content {
        header: "header".to_string(),
        body: "body".to_string(),
    };

    assert_eq!(expected, ctx.hash().unwrap().to_string());

    let database_object = DatabaseObject::from_content(ctx);
    let hash2 = GenericHash::from_bytes(&database_object.hash).to_string();

    assert_eq!(expected, hash2.to_string());
}

#[test]
fn relation_hash() {
    let expected = "03a73b0774eaa16a75930737fc4c317da50bd9769bd983bea89c23c6f0e5873c";

    let rel = RelationBase {
        definition: GenericHash::from_hex(
            "1111111111111111111111111111111111111111111111111111111111111111",
        ),
        first_object: GenericHash::from_hex(
            "2222222222222222222222222222222222222222222222222222222222222222",
        ),
        second_object: GenericHash::from_hex(
            "3333333333333333333333333333333333333333333333333333333333333333",
        ),
    };

    assert_eq!(expected, rel.hash().unwrap().to_string());

    let rel2 = RelationBase {
        definition: GenericHash::from_hex(
            "1111111111111111111111111111111111111111111111111111111111111111",
        ),
        first_object: GenericHash::from_hex(
            "3333333333333333333333333333333333333333333333333333333333333333",
        ),
        second_object: GenericHash::from_hex(
            "2222222222222222222222222222222222222222222222222222222222222222",
        ),
    };

    assert_eq!(expected, rel2.hash().unwrap().to_string());
}
