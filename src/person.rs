use std::fmt;

use sodiumoxide::crypto;
use key_pair::{ KeyPair };

use serde::ser::{ Serialize, Serializer, SerializeStruct };


pub struct Person {
	name: String,
	keys: KeyPair
}

impl Person {
	pub fn get_name(&self) -> &str {
		return &self.name;
	}
	pub fn sign_msg(&self, message: &str) -> crypto::sign::Signature {
		return crypto::sign::sign_detached(message.as_bytes(), &self.keys.secret_key);
	}
	pub fn verify(&self, message: &str, signature: &crypto::sign::Signature) -> bool {
		return crypto::sign::verify_detached(signature, message.as_bytes(), &self.keys.public_key);
	}
}

impl fmt::Display for Person {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "name: {}, keys: {}", self.name, self.keys)
	}
}

pub fn create_person(name: &str) -> Person {
	let (pk, sk) = crypto::sign::gen_keypair();
	Person { name: name.to_string(), keys: KeyPair { public_key: pk, secret_key: sk } }
}

impl Serialize for Person {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		// 3 is the number of fields in the struct.
		let mut state = serializer.serialize_struct("Person", 2)?;
		state.serialize_field("name", &self.name)?;
		state.serialize_field("keys", &self.keys)?;
		state.end()
	}
}
