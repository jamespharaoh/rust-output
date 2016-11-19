pub trait Output {

	fn status (
		& mut self,
		status: & str,
	);

	fn message (
		& mut self,
		message: & str,
	);

	fn clear_status (
		& mut self,
	);

}

// ex: noet ts=4 filetype=rust
