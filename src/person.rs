use std::fmt;

use sodiumoxide::crypto;
use key_pair::{ KeyPair };


#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
	name: String,
	keys: KeyPair
}

impl Person {
	pub fn get_name(&self) -> &str {
		&self.name
	}
	pub fn get_pubkey(&self) -> &crypto::sign::PublicKey {
		&self.keys.public_key
	}
	pub fn sign_msg(&self, message: &str) -> crypto::sign::Signature {
		crypto::sign::sign_detached(message.as_bytes(), &self.keys.secret_key)
	}
	pub fn verify(&self, message: &str, signature: &crypto::sign::Signature) -> bool {
		crypto::sign::verify_detached(signature, message.as_bytes(), &self.keys.public_key)
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
