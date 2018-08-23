use std::fmt;
use sodiumoxide::crypto::sign;

use sodiumoxide::crypto::secretbox;
use sodiumoxide::crypto::pwhash;
use bincode;

use error::Error;
use utils::{ to_hex_string };


pub fn pubkey_to_str(pubkey: &sign::PublicKey) -> String {
	let mut hex_string = String::new();

	for byte in pubkey.as_ref() {
		hex_string += &format!("{:x}", byte);
	}
	hex_string
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PersonalKeys {
	// singing public key
	pub pk: sign::PublicKey,

	// password hash
	pwh: pwhash::HashedPassword,

	// secret key derive salt
	salt: pwhash::Salt,

	// secret box nonce
	nonce: secretbox::Nonce,

	// encrypted secret
	encrypted_secret: Vec<u8>
}

impl fmt::Display for PersonalKeys {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// writting only public key
		write!(f, "public {}",
			to_hex_string(self.pk.as_ref()))
	}
}

impl PersonalKeys {
	pub fn new(password: &str) -> PersonalKeys {
		let passwd = PersonalKeys::make_passwd(password);
		let pwh = PersonalKeys::hash_password(&passwd);
		let nonce = secretbox::gen_nonce();

		let (pk, sk) = sign::gen_keypair();
		let (encrypted_secret, salt) = Self::encrypt_sign_key(&sk, &nonce, &passwd);

		PersonalKeys { pk, pwh, salt, nonce, encrypted_secret }
	}

	pub fn sign(&self, message: &str, password: &str) -> Result<sign::Signature, Error> {
		let sk = Self::decrypt_sign_key(self, &PersonalKeys::make_passwd(&password))?;

		let signature = sign::sign_detached(
			message.as_bytes(),
			&sk
		);

		Ok(signature)
	}

	pub fn verify(&self, message: &str, signature: &sign::Signature) -> Result<(), Error> {
		match sign::verify_detached(signature, message.as_bytes(), &self.pk) {
			true => Ok(()),
			false => Err(Error::SignVerifyFail)
		}
	}

	fn make_passwd(password: &str) -> Vec<u8> {
		format!("c@p)*c=#e{}", password).into_bytes()
	}

	fn hash_password(passwd: &[u8]) -> pwhash::HashedPassword {
		let pwh = pwhash::pwhash(passwd,
		                         pwhash::OPSLIMIT_INTERACTIVE,
		                         pwhash::MEMLIMIT_INTERACTIVE).unwrap();
		pwh
	}

	fn verify_password(&self, passwd: &[u8]) -> Result<(), Error> {
		match pwhash::pwhash_verify(&self.pwh, passwd) {
			true => Ok(()),
			false => Err(Error::WrongPassword)
		}
	}

	fn derive_key(passwd: &[u8], salt: &pwhash::Salt) -> secretbox::Key {
		let mut k = secretbox::Key([0; secretbox::KEYBYTES]);
		{
			let secretbox::Key(ref mut kb) = k;
			pwhash::derive_key(kb, passwd, &salt,
			                   pwhash::OPSLIMIT_INTERACTIVE,
			                   pwhash::MEMLIMIT_INTERACTIVE).unwrap();
		}
		k
	}

	fn encrypt_sign_key(sk: &sign::SecretKey, nonce: &secretbox::Nonce, passwd: &[u8]) -> (Vec<u8>, pwhash::Salt) {
		let salt = pwhash::gen_salt();
		let encryption_key = Self::derive_key(&passwd, &salt);

		let sk_bytes = bincode::serialize(&sk).unwrap();
		let encrypted_secret = secretbox::seal(&sk_bytes, &nonce, &encryption_key);

		(encrypted_secret, salt)
	}

	fn decrypt_sign_key(&self, passwd: &[u8]) -> Result<sign::SecretKey, Error> {
		self.verify_password(passwd)?;

		let encryption_key = Self::derive_key(&passwd, &self.salt);
		let sk_bytes = secretbox::open(&self.encrypted_secret, &self.nonce, &encryption_key).unwrap();

		let sk: sign::SecretKey = bincode::deserialize(&sk_bytes).unwrap();

		Ok(sk)
	}
}
