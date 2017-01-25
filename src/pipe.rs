use std::fmt;
use std::io;
use std::io::Write;

use backend::Backend;

pub struct PipeOutput {
	error_handler: Box <Fn (io::Error) + Send>,
}

impl PipeOutput {

	pub fn new (
		error_handler: Box <Fn (io::Error) + Send>,
	) -> PipeOutput {

		PipeOutput {
			error_handler: error_handler,
		}

	}

}


impl Backend for PipeOutput {

	fn message_format (
		& mut self,
		message_arguments: fmt::Arguments,
	) {

		// print message

		io::stderr ().write_fmt (
			format_args! (
				"{}\n",
				message_arguments)
		).unwrap_or_else (
			|error|

			(self.error_handler) (
				error)

		);

	}

	fn status_format (
		& mut self,
		_status_arguments: fmt::Arguments,
	) {

		// do nothing

	}

	fn clear_status (
		& mut self,
	) {

		// do nothing

	}

	fn status_progress (
		& mut self,
		_numerator: u64,
		_denominator: u64,
	) {

		// do nothing

	}

	fn status_tick (
		& mut self,
	) {

	}

	fn status_done (
		& mut self,
	) {

		// do nothing

	}

}

// ex: noet ts=4 filetype=rust
