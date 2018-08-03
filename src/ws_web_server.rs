use serde_json;
// A WebSocket ws-rs server
use ws;
use ws::{ listen, CloseCode, Handler, Message, Handshake, Sender};

use accounts_storage::{ AccountsStorage, create_storage };

use error::StringError;

fn parse_json_cmd(data: &str) -> Result<serde_json::Value, String> {
	// Parse the string of data into serde_json::Value.
	// let data2 = r#" { "name": "John Doe", "age": 43, "phones": ["+44 1234567"] } "#;
	// let v = serde_json::from_str(data);
	match serde_json::from_str(data) {
		Ok(value) => Ok(value),
		Err(error) => Err(format!("Fail to parse JSON command: {:?}, error: {:?}", data, error))
	}
}

// return key value if ok, otherwise sending error message
// require string values
fn require_json_key(json: &serde_json::Value, key: &str, error_out: &Sender) -> Result<String, StringError> {
	let key_value: Option<String>;
	if json[key].is_string() {
		let value = json[key].as_str().unwrap();
		key_value = Some(<String>::from(value));
	}
	else {
		key_value = None;
	}

	match key_value {
		Some(value) => Ok(value),
		None => {
			let err_msg = format!("key {:?} as string is required!", key);
			error_out.send(err_msg.clone()).unwrap();
			Err(StringError::new(&err_msg))
		}
	}
}

pub fn create_web_server(url: &str) -> ws::Result<()> {
	println!("Creating webSocket server, url: {}", url);

	struct Server {
		out: Sender,
		main_storage: AccountsStorage
	}

	impl Handler for Server {
		fn on_open(&mut self, shake: Handshake) -> ws::Result<()> {
			if let Some(ip_addr) = shake.remote_addr()? {
				println!("Connection opened from {}.", ip_addr)
			} else {
				println!("Unable to obtain client's IP address.")
			}
			Ok(())
		}

		fn on_message(&mut self, msg: Message) -> ws::Result<()> {
			// Handle messages received on this connection
			println!("Server got message '{}'. ", msg);

			let str_msg = msg.as_text().unwrap();

			match parse_json_cmd(str_msg) {
				Ok(value) => {
					println!("FULL CMD JSON is: {:?}", value);

					let cmd = &value["cmd"];
					if cmd.is_string() {
						let cmd = cmd.as_str().unwrap();
						match cmd {
							"create_account" => {
								println!("Got AA!");
								let name;
								let pass;
								if let Ok(name_value) = require_json_key(&value, "name", &self.out) {
									name = name_value;
									if let Ok(pass_value) = require_json_key(&value, "password", &self.out) {
										pass = pass_value;
										println!("cmd: {:?}, name: {:?}, pass: {:?}", cmd, name, pass);
										self.main_storage.new_person(&pass, &name);
									}
								}
							},
							"show_storage" => {
								self.main_storage.show_all();
							}
							_ => {
								let response = "Unknown command: ".to_owned() + cmd;
								self.out.send(response).unwrap();
							}
						};
					} else {
						let response = value["cmd"].to_string() + " is a bad command";
						self.out.send(response).unwrap();
					}
				},
				Err(error) => {
					println!("{:?}", error);
					let response = "Bad command: ".to_owned() + str_msg;
					self.out.send(response).unwrap();
				}
			};
			Ok(())
		}

		fn on_close(&mut self, code: CloseCode, reason: &str) {
			println!("WebSocket closing for ({:?}) {}", code, reason);
			println!("Shutting down server after first connection closes.");
			self.out.shutdown().unwrap();
		}
	}

	// create accounts storage
	// returning Server listener
	return listen(url, |out| Server { out, main_storage: create_storage(10) });
}

/*	EXAMPLE WITH ERROR HANDLING
	// Listen on an address and call the closure for each connection
	if let Err(error) = listen("127.0.0.1:3012", |out| {
		// The handler needs to take ownership of out, so we use move
		move |msg| {
			// Handle messages received on this connection
			println!("Server got message '{}'. ", msg);

			// Use the out channel to send messages back
			out.send(msg)
		}
	}) {
		// Inform the user of failure
		println!("Failed to create WebSocket due to {:?}", error);
	}
*/
