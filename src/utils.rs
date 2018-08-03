pub fn to_hex_string(bytes: &[u8]) -> String {
	let mut hex_string = String::new();
	for byte in bytes {
		hex_string += &format!("{:02x}", byte);
	}
	hex_string
}
