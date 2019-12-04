use std::fmt;

use crypto::personal_keys::PersonalKeys;
use sodiumoxide::crypto;

use error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    name: String,
    keys: PersonalKeys,
}

impl Person {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_pubkey(&self) -> &crypto::sign::PublicKey {
        &self.keys.pk
    }
    pub fn sign_msg(
        &self,
        message: &str,
        password: &str,
    ) -> Result<crypto::sign::Signature, Error> {
        self.keys.sign(&message, &password)
    }
    pub fn verify(&self, message: &str, signature: &crypto::sign::Signature) -> Result<(), Error> {
        self.keys.verify(&message, &signature)
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {}, keys: {}", self.name, self.keys)
    }
}

pub fn create_person(name: &str, password: &str) -> Person {
    Person {
        name: name.to_string(),
        keys: PersonalKeys::new(&password),
    }
}
