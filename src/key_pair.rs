use std::fmt;
use sodiumoxide::crypto;

use utils::{ to_hex_string };


#[derive(Serialize, Deserialize, Debug)]
pub struct KeyPair {
	pub public_key: crypto::sign::PublicKey,
	pub secret_key: crypto::sign::SecretKey
}

impl fmt::Display for KeyPair {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "public {}, secret: {:?}",
			to_hex_string(self.public_key.as_ref()),
			self.secret_key)
	}
}
