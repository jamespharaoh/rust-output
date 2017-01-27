use std::fmt::Write as FormatWrite;
use std::io;
use std::io::Write;

use termion;

use backend::*;
use output::*;

pub struct Console <'a> {
	status_tick_sequence: & 'a [String],
	error_handler: Box <Fn (io::Error) + Send>,
	columns: u16,
	status_lines: u16,
}

impl <'a> Console <'a> {

	pub fn new (
		error_handler: Box <Fn (io::Error) + Send>,
		status_tick_sequence: & 'a [String],
	) -> Console <'a> {

		let columns =
			match termion::terminal_size () {

			Ok ((columns, _rows)) =>
				columns,

			Err (_) => 80,

		};

		Console {
			status_tick_sequence: status_tick_sequence,
			error_handler: error_handler,
			columns: columns,
			status_lines: 0,
		}

	}

	fn write_message (
		& self,
		target: & mut FormatWrite,
		message: & str,
	) {

		write! (
			target,
			"{}{}\r\n",
			if message.len () <= self.columns as usize {
				& message
			} else {
				& message [0 .. self.columns as usize]
			},
			termion::clear::AfterCursor,
		).unwrap ();

	}

	fn write_running (
		& self,
		target: & mut FormatWrite,
		message: & str,
		status: Option <& str>,
	) {

		if let Some (status) = status {

			write! (
				target,
				"{} ... {}{}\r\n",
				if message.len () <= self.columns as usize - status.len () - 5 {
					& message
				} else {
					& message [0 .. self.columns as usize - status.len () - 5]
				},
				status,
				termion::clear::AfterCursor,
			).unwrap ();

		} else {

			write! (
				target,
				"{} ...{}\r\n",
				if message.len () <= self.columns as usize - 4 {
					& message
				} else {
					& message [0 .. self.columns as usize - 4]
				},
				termion::clear::AfterCursor,
			).unwrap ();

		}

	}

}

impl <'a> Backend for Console <'a> {

	fn update (
		& mut self,
		logs: & [OutputLogInternal],
	) {

		let mut buffer =
			String::new ();

		// move up to the start

		if self.status_lines > 0 {

			write! (
				buffer,
				"\r{}",
				termion::cursor::Up (
					self.status_lines),
			).unwrap ();

		}

		// output logs

		let old_status_lines = self.status_lines;
		self.status_lines = 0;

		for log in logs {

			if log.state () == OutputLogState::Removed {
				continue;
			}

			if log.state () == OutputLogState::Running
				|| self.status_lines > 0 {

				self.status_lines += 1;

			}

			if log.state () == OutputLogState::Running {

				if log.denominator () > 0 {

					let percent_string =
						format! (
							"{}%",
							log.numerator () * 100 / log.denominator ());

					self.write_running (
						& mut buffer,
						log.message (),
						Some (& percent_string));

				} else if log.tick () > 0 {

					let tick_string =
						& self.status_tick_sequence [
							(log.tick () as usize - 1)
								% self.status_tick_sequence.len ()];

					self.write_running (
						& mut buffer,
						log.message (),
						Some (& tick_string));

				} else {

					self.write_running (
						& mut buffer,
						log.message (),
						None);

				}

			} else if log.state () == OutputLogState::Complete {

				self.write_running (
					& mut buffer,
					log.message (),
					Some ("done"));

			} else if log.state () == OutputLogState::Incomplete {

				self.write_running (
					& mut buffer,
					log.message (),
					Some ("abort"));

			} else if log.state () == OutputLogState::Message {

				self.write_message (
					& mut buffer,
					log.message ());

			} else {

				unreachable! ();

			}

		}

		if self.status_lines < old_status_lines {

			for _index in 0 .. (old_status_lines - self.status_lines) {

				write! (
					buffer,
					"{}\n",
					termion::clear::CurrentLine,
				).unwrap ();

			}

			write! (
				buffer,
				"{}",
				termion::cursor::Up (
					old_status_lines - self.status_lines),
			).unwrap ();

		}

		write! (
			io::stderr (),
			"{}",
			buffer,
		).unwrap_or_else (
			|error|

			(self.error_handler) (
				error)

		);

	}

}

// ex: noet ts=4 filetype=rust
