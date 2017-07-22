use output_log::*;

pub type BoxBackend = Box <Backend>;

pub trait Backend: Send {

	fn update (
		& mut self,
		jobs: & [OutputLogInternal],
	);

	fn synchronous (& self) -> bool;

}

// ex: noet ts=4 filetype=rust
