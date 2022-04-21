pub struct Base {
	pub title: String,
	pub status: String,
}

impl Base {
	pub fn new(title: &str, status: &str) -> Base {
		Base {
			title: title.to_owned(),
			status: status.to_owned(),
		}
	}
}
