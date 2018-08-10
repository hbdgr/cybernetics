use std::fmt;
use sodiumoxide::crypto;
use std::collections::HashMap;

use std::fs::File;

use person::{ Person, create_person };

use bincode;

use utils::{ to_hex_string };


#[derive(Serialize, Deserialize, Debug)]
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
	pub fn save_to_file(&self) -> Result<(), bincode::Error> {
		let mut file = File::create("storage.dat")?;

		bincode::serialize_into(&mut file, &self)?;
		Ok(())
	}

	pub fn load_from_file() -> Result<AccountsStorage, bincode::Error> {
		let mut file = File::open("storage.dat")?;

		// let mut decoded: AccountsStorage = bincode::deserialize_from(&mut file).unwrap();
		// self = &mut decoded;
		let decoded: AccountsStorage = bincode::deserialize_from(&mut file)?;
		println!("Deserialized acc {:?}", decoded);
		Ok(decoded)
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

pub fn restore_storage(capacity: usize) -> AccountsStorage {
	let storage: AccountsStorage;
	match AccountsStorage::load_from_file() {
		Ok(value) => {
			storage = value;
			println!("Loaded storage from file!");
		},
		Err(err) => {
			storage = AccountsStorage { accounts: HashMap::with_capacity(capacity) };
			println!("Fail to load storage from file {:?}", err);
			println!("Returinig empty storage...");
		}
	}
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
