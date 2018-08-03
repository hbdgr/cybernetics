extern crate sodiumoxide;
extern crate ws;

extern crate serde;
extern crate serde_json;
extern crate bincode;


mod person;
mod key_pair;
mod accounts_storage;
mod msg_block;
mod ws_web_server;
mod error;
mod utils;

use std::thread;
use std::time::Duration;

fn main() {
	// run server!
	let server_thread = thread::spawn(move ||
		ws_web_server::create_web_server("127.0.0.1:3012").unwrap());

	// Give the server a little time to get going
	thread::sleep(Duration::from_millis(10));

	let first_struct = msg_block::build_msg_block("author", "example message");
	println!("MessageBlock: {}", first_struct);

	let example_person = person::create_person("Amelino");
	println!("First user name: {}", example_person.get_name());

	let msg_to_sign = "example message to sign";

	let signature = example_person.sign_msg(msg_to_sign);
	assert!(example_person.verify(msg_to_sign, &signature));

	let _ = server_thread.join();
}
