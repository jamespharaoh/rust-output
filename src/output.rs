pub trait Output {

	fn message (
		& mut self,
		message: & str,
	);

	fn status (
		& mut self,
		status: & str,
	);

	fn clear_status (
		& mut self,
	);

	fn status_done (
		& mut self,
	);

}

pub type OutputBox = Box <Output>;

// ex: noet ts=4 filetype=rust
