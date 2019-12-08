use crypto::strings;
use hex::FromHex;
use serde::ser::{Serialize, Serializer};
use sodiumoxide::crypto::generichash::State;
use std::convert::TryInto;
use std::fmt;

const GENERIC_HASH_SIZE: usize = 32;

pub struct GenericHash {
    bytes: [u8; GENERIC_HASH_SIZE],
}

impl GenericHash {
    fn to_string(&self) -> String {
        strings::to_hex_string(&self.bytes)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }

    pub fn from_hex(s: &str) -> GenericHash {
        let v: Vec<u8> = Vec::from_hex(s).expect("GenericHash::from_hex: Invalid Hex String");
        let slice: &[u8] = &v;
        let array: [u8; GENERIC_HASH_SIZE] = slice
            .try_into()
            .expect("GenericHash::from_hex: Incorrect length of bytes slice");

        GenericHash { bytes: array }
    }

    pub fn from_bytes(b: &[u8]) -> GenericHash {
        let array: [u8; GENERIC_HASH_SIZE] = b
            .try_into()
            .expect("GenericHash::from_bytes: Incorrect length of bytes slice");
        GenericHash { bytes: array }
    }
}

impl Default for GenericHash {
    fn default() -> Self {
        GenericHash::from_hex("0000000000000000000000000000000000000000000000000000000000000000")
    }
}

impl Serialize for GenericHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl fmt::Display for GenericHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub fn generic_state() -> Result<State, ()> {
    let hasher = State::new(GENERIC_HASH_SIZE, None)?;
    Ok(hasher)
}

pub fn generic_finalize(hasher: State) -> Result<Vec<u8>, ()> {
    let finalized = hasher.finalize()?;

    Ok(Vec::from(finalized.as_ref()))
}

pub fn raw_generic(bytes: &[u8]) -> Result<Vec<u8>, ()> {
    let mut hasher = generic_state()?;
    hasher.update(bytes)?;

    Ok(generic_finalize(hasher)?)
}
