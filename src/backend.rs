use output::*;

pub type BoxBackend = Box <Backend>;

pub trait Backend: Send {

	fn update (
		& mut self,
		jobs: & [OutputLogInternal],
	);

}

// ex: noet ts=4 filetype=rust
