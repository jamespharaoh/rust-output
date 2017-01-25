use std::fmt;

pub type BoxBackend = Box <Backend>;

pub trait Backend: Send {

	fn message_format (
		& mut self,
		message_arguments: fmt::Arguments,
	);

	fn status_format (
		& mut self,
		status_arguments: fmt::Arguments,
	);

	fn clear_status (
		& mut self,
	);

	fn status_progress (
		& mut self,
		numerator: u64,
		denominator: u64,
	);

	fn status_tick (
		& mut self,
	);

	fn status_done (
		& mut self,
	);

}

// ex: noet ts=4 filetype=rust
