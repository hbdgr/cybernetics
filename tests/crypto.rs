extern crate cybernetics;

use cybernetics::crypto::{msg_block, person};

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
