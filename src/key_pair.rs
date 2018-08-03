use std::fmt;
use sodiumoxide::crypto;

use serde::ser::{ Serialize, Serializer, SerializeStruct };

use utils::{ to_hex_string };

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

impl Serialize for KeyPair {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut state = serializer.serialize_struct("KeyPair", 2)?;
		state.serialize_field("pubkey", &self.public_key)?;
		state.serialize_field("privkey", &self.secret_key)?;
		state.end()
	}
}
