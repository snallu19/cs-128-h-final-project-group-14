pub struct TinyEncrypAlg {
	pub message: String,
	pub key_chars: Vec<char>(4),
	pub encrypted: String,
	const delta: u32 = 0x9e3779b9;
}

impl TinyEncrypAlg {
	pub fn new(message: String, key: Vect<u32>) -> TinyEncryAlg {
		let key_chars = key.copy().chars().collect();
		TinyEncrypAlg{message, key_chars, encrypted: String::new()}	
	}

	pub fn encrypt {
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
			let mut sum = 0;
			for j in range(32) {
				sum += delta;
				v0 += ((v1 << 4) + key_chars[0]) ^ (v1 + sum) ^ ((v1 >> 5) + key_chars[1]);
				v1 += ((v0 << 4) + key_chars[2]) ^ (v0 + sum) ^ ((v0 >> 5) + key_chars[3]);
			}

		}
		encrypted = x.into_iter().flatten().collect();
	}
}