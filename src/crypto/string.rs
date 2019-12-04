use sodiumoxide::crypto::sign;

pub fn pubkey_to_string(pubkey: &sign::PublicKey) -> String {
    let mut hex_string = String::new();

    for byte in pubkey.as_ref() {
        hex_string += &format!("{:x}", byte);
    }
    hex_string
}

pub fn to_hex_string(bytes: &[u8]) -> String {
    let mut hex_string = String::new();
    for byte in bytes {
        hex_string += &format!("{:02x}", byte);
    }
    hex_string
}
