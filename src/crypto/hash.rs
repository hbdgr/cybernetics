use crypto::strings;
use hex::FromHex;
use serde::de::{self, Deserialize, Deserializer, Unexpected, Visitor};
use serde::ser::{Serialize, Serializer};
use sodiumoxide::crypto::generichash::State;
use std::convert::TryInto;
use std::fmt;

const GENERIC_HASH_SIZE: usize = 32;

#[derive(Clone)]
pub struct GenericHash {
    bytes: [u8; GENERIC_HASH_SIZE],
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

impl GenericHash {
    pub fn new(bytes: &[u8]) -> Result<Self, ()> {
        let v = raw_generic(bytes)?;
        let slice: &[u8] = &v;
        let array: [u8; GENERIC_HASH_SIZE] = slice
            .try_into()
            .expect("GenericHash::from_hex: Incorrect length of bytes slice");
        Ok(Self { bytes: array })
    }

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

impl<'de> Deserialize<'de> for GenericHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GH;

        impl<'de> Visitor<'de> for GH {
            type Value = GenericHash;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    formatter,
                    "a string in hex format containing {} chars",
                    GENERIC_HASH_SIZE * 2
                )
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if s.len() == GENERIC_HASH_SIZE * 2 {
                    Ok(GenericHash::from_hex(s))
                } else {
                    Err(de::Error::invalid_value(Unexpected::Str(s), &self))
                }
            }
        }
        deserializer.deserialize_str(GH)
    }
}
