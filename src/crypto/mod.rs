pub mod hash;
pub mod msg_block;
pub mod person;
pub mod strings;

mod personal_keys;

pub fn init() {
    sodiumoxide::init().expect("Sodium failed to init");
}
