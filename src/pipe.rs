use output::Output;

pub struct PipeOutput {
}

impl PipeOutput {

	pub fn new (
	) -> PipeOutput {

		PipeOutput {
		}

	}

}


impl Output for PipeOutput {

	fn status (
		& mut self,
		_status: & str,
	) {

		// do nothing

	}

	fn message (
		& mut self,
		message: & str,
	) {

		// print message

		stderr! (
			"{}\n",
			message);

	}

	fn clear_status (
		& mut self,
	) {

		// do nothing

	}

	fn status_done (
		& mut self,
	) {

		// do nothing

	}

}

// ex: noet ts=4 filetype=rust
