use std::fmt;

pub struct MessageBlock {
	message: String,
	author: String
}

impl fmt::Display for MessageBlock {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}, author: {}", self.message, self.author)
	}
}

pub fn build_msg_block(author: &str, message: &str) -> MessageBlock {
	MessageBlock {
		author: author.to_string(),
		message: message.to_string()
	}
}
