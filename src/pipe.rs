use std::io;
use std::io::Stdout;

use output::Output;

pub struct PipeOutput {
	stdout: Stdout,
}

impl PipeOutput {

	pub fn new (
	) -> PipeOutput {

		PipeOutput {
			stdout: io::stdout (),
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
