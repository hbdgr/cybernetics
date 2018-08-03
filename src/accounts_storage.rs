use std::io;
use std::fmt;
use sodiumoxide::crypto;
use std::collections::HashMap;

use std::fs::File;

use person::{ Person, create_person };

use bincode;
use serde::ser::{ Serialize, Serializer, SerializeMap };
// use serde::de::{ Deserialize, Deserializer };

use utils::{ to_hex_string };


pub struct AccountsStorage {
	accounts: HashMap<crypto::hash::Digest, Person>
}

impl AccountsStorage {
	// mixing password with salt to lower password duplication recognision probability
	fn create_mixed_hashkey(ps: &str, salt: &str) -> crypto::hash::Digest {
		let mixed_ps = ps.to_owned() + &salt;
		crypto::hash::hash(mixed_ps.as_bytes())
	}
/*
	pub fn add_person(&mut self, password: &str, person: Person) {
		let hash_key = AccountsStorage::create_mixed_hashkey(password, &person.name);
		&self.accounts.insert(hash_key, person);
	}
*/
	pub fn save_to_file(&self) -> io::Result<()> {
		let mut file = File::create("storage.dat")?;

		let serialized = bincode::serialize_into(&mut file, &self).unwrap();

		println!("Serialized?: {:?}", &serialized);
		Ok(())
	}

	pub fn load_from_file(&self) -> io::Result<()> {
		let mut file = File::open("storage.dat")?;

		// let decoded: AccountsStorage = bincode::deserialize_from(&mut file).unwrap();
		// self = &decoded;
		Ok(())
	}

	pub fn new_person(&mut self, password: &str, person_name: &str) {
		let person = create_person(&person_name);

		let hash_key = AccountsStorage::create_mixed_hashkey(password, person_name);
		&self.accounts.insert(hash_key, person);

		&self.save_to_file();
	}

	pub fn show_all(&self) {
		println!("{}", &self);
	}
}

pub fn create_storage(capacity: usize) -> AccountsStorage {
	let storage = AccountsStorage { accounts: HashMap::with_capacity(capacity) };
	storage.load_from_file();
	storage
}

impl fmt::Display for AccountsStorage {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut my_accounts_info = String::new();

		// iterate over all records.
		let mut i = 0;
		for (hash, person) in &self.accounts {
			my_accounts_info += &format!("({}) {}: {}\n", i, to_hex_string(hash.as_ref()), person);
			i += 1;
		}
		write!(f, "Accounts ({}):\n{}", self.accounts.len(), my_accounts_info)
	}
}

impl Serialize for AccountsStorage {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut map = serializer.serialize_map(Some(self.accounts.len()))?;
		for (k, v) in &self.accounts {
			map.serialize_entry(k.as_ref(), &v)?;
		}
		map.end()
	}
}

/*
impl<'de> Deserialize<'de> for AccountsStorage {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let storage = create_storage(1);
		storage.accounts = deserializer.deserialize_map()?;
		Ok(storage)
	}
}
*/
