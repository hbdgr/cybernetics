#![feature(plugin, decl_macro, custom_derive, extern_prelude)]
#![plugin(rocket_codegen)]

// mute warnings, couse by compiler, should be fixed in rustc 1.4
// https://github.com/diesel-rs/diesel/issues/1785
#![allow(proc_macro_derive_resolution_fallback)]

extern crate sodiumoxide;
extern crate ws;
extern crate serde;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate bincode;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;


mod person;
mod crypto;
mod accounts_storage;
mod msg_block;
mod ws_web_server;
mod error;
mod utils;
mod database;
mod server;

use std::thread;
use std::time::Duration;

use dotenv::dotenv;


fn main() {
	dotenv().ok();
	// run rest server!

	let rest_thread = thread::spawn(move ||
		server::router::create_routes());
	println!("REST API created");

	// run ws server!
	let ws_server_thread = thread::spawn(move ||
		ws_web_server::create_web_server("127.0.0.1:3012").unwrap());

	// Give the servers a little time to get going
	thread::sleep(Duration::from_millis(10));

	let first_struct = msg_block::build_msg_block("author", "example message");
	println!("MessageBlock: {}", first_struct);

	let example_pass = "very_hard_to_breake_password";
	let example_person = person::create_person("Amelino", example_pass);
	println!("First user name: {}", example_person.get_name());

	let msg_to_sign = "example message to sign";

	let signature = example_person.sign_msg(msg_to_sign, example_pass).unwrap();
	example_person.verify(msg_to_sign, &signature).unwrap();

	let _ = rest_thread.join();
	let _ = ws_server_thread.join();
}
