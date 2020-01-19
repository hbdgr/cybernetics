extern crate cybernetics;
extern crate serde_json;

mod common;

use common::object_helpers;
use cybernetics::crypto::{hash, msg_block, person, strings};
use cybernetics::database::object::DatabaseObject;
use cybernetics::database::relation::DatabaseRelation;
use cybernetics::primitives::header::ObjectType;
use cybernetics::primitives::object::Object;
use cybernetics::primitives::relation::{Relation, RelationBase};
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
    let expected = "ebcc521a3b0a7eb3df607a9ad2cea3659c77762642c2d13ca84d3ba44f40ead3";

    let ctx = object_helpers::test_content(ObjectType::PrimaryElement, "body");

    assert_eq!(expected, ctx.hash().unwrap().to_string());

    let object = Object::from_content(ctx.clone()).unwrap();

    let hash2 = object.hash.to_string();
    assert_eq!(expected, hash2.to_string());

    let database_object = DatabaseObject::from_content(ctx);
    let hash3 = GenericHash::from_bytes(&database_object.hash).to_string();

    assert_eq!(expected, hash3.to_string());
}

#[test]
fn relation_hash() {
    let expected = "ddf9ac032fce3c93cd100c177c408e7b8e4886eab249a7efcf0484b7c080c5ec";
    let undirected = false;
    let def = object_helpers::test_relation_definition(undirected, "def_relation_hash");
    let first_obj = object_helpers::test_primary_element("first_relation_hash");
    let second_obj = object_helpers::test_primary_element("second_relation_hash");

    let rel = RelationBase {
        definition: def.clone(),
        first_object: first_obj.clone(),
        second_object: second_obj.clone(),
    };

    assert_eq!(expected, rel.hash().unwrap().to_string());

    let rel2 = RelationBase {
        definition: def,
        first_object: second_obj,
        second_object: first_obj,
    };

    assert_eq!(expected, rel2.hash().unwrap().to_string());
}

#[test]
fn relation_hash_conversion() {
    let expected = "99171f4784aa1b0f98637cd7c361795f34a2bd65c9f37fa2ef0a3e28174edb09";
    let undirected = false;
    let def = object_helpers::test_relation_definition(undirected, "def_relation_hash_conversion");
    let first_obj = object_helpers::test_primary_element("first_hash_conversion");
    let second_obj = object_helpers::test_primary_element("second_hash_conversion");

    let rel = RelationBase {
        definition: def,
        first_object: first_obj,
        second_object: second_obj,
    };

    assert_eq!(expected, rel.hash().unwrap().to_string());

    let relation = Relation::from_relation_base(rel.clone()).unwrap();
    let hash2 = relation.hash.to_string();
    assert_eq!(expected, hash2.to_string());

    let database_relation = DatabaseRelation::from_relation_base(rel);
    let hash3 = GenericHash::from_bytes(&database_relation.hash).to_string();
    assert_eq!(expected, hash3.to_string());
}
