use std::io;
use std::io::Write;

use backend::*;
use output_log::*;

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

	fn update (
		& mut self,
		logs: & [OutputLogInternal],
	) {

		for log in logs {

			if log.state () != OutputLogState::Message {
				break;
			}

			write! (
				io::stderr (),
				"{}\n",
				log.message (),
			).unwrap_or_else (
				|error|

				(self.error_handler) (
					error)

			);

		}

	}

}

// ex: noet ts=4 filetype=rust
