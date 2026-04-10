pub struct TinyDecrypAlg {
	pub message: String,
	pub key_chars: Vec<chars>(4),
	pub decrypted: String,
	const delta: u32 = 0x9e3779b9,
}

impl TinyDecrypAlg {
	pub fn new(message, key) -> TinyDecrypAlg {
		let key_chars = key.copy().chars().collect();
		TinyDecrypAlg{message, key_chars, decrypted: String::new()}
	}

	pub fn decrypt {
		let message_chars = message.copy().chars().collect();
		let mut x = Vec<Vec<String>>;
		let mut index = 0;
		for i in range(6) {
			x[i] = message_chars.split_off(index + 32);
			index += 32;
		}
		for i in range(6).step_by(2) {
			let mut v0 = &mut x[i];
			let mut v1 = &mut x[i + 1];
			let mut sum = delta << 5;
			for j in range(32) {
				v1 -= ((v0 << 4) + key_chars[2]) ^ (v0 + sum) ^ ((v0 >> 5) + key_chars[3]);
				v0 -= ((v1 << 4) + key_chars[0]) ^ (v1 + sum) ^ ((v1 >> 5) + key_chars[1]);
				sum -= delta;
			}
		}
		decrypted = x.into_iter().flatten().collect();
	}
}