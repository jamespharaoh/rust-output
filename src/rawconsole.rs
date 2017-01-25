use std::fmt;
use std::io;
use std::io::Stderr;
use std::io::Write;
use std::thread;
use std::thread::JoinHandle;

use libc;

use termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

use backend::Backend;

pub struct RawConsole <'a> {
	error_handler: Box <Fn (io::Error) + Send>,
	_output: RawTerminal <Stderr>,
	columns: u64,
	status: Option <String>,
	status_suffix: Option <String>,
	status_tick: u64,
	status_tick_sequence: & 'a Vec <String>,
	_input_thread: JoinHandle <()>,
}

impl <'a> RawConsole <'a> {

	pub fn new (
		error_handler: Box <Fn (io::Error) + Send>,
		status_tick_sequence: & 'a Vec <String>,
	) -> Option <RawConsole <'a>> {

		// setup output

		let output =
			match io::stderr ().into_raw_mode () {

			Ok (terminal) =>
				terminal,

			Err (_) =>
				return None,

		};

		let columns: u64 =
			match termion::terminal_size () {

			Ok ((columns, _rows)) =>
				columns as u64,

			Err (_) => 80,

		};

		// setup input

		let input_thread =
			thread::spawn (
				|| Self::input_thread ());

		Some (
			RawConsole {

				error_handler: error_handler,

				_output: output,
				columns: columns,

				status: None,
				status_suffix: None,
				status_tick: 0,
				status_tick_sequence: status_tick_sequence,

				_input_thread: input_thread,

			}
		)

	}

	fn input_thread (
	) {

		let stdin =
			io::stdin ();

		for key_result in stdin.keys () {

			if let Ok (key) = key_result {

				match key {

					Key::Ctrl ('c') => {

						unsafe {

							libc::kill (
								libc::getpid (),
								libc::SIGINT);

						}

					},

					Key::Ctrl ('z') => {

						unsafe {

							libc::kill (
								libc::getpid (),
								libc::SIGSTOP);

						}

					},

					_ => {
						// ignore
					},

				}

			}

		}

	}

}

impl <'a> Backend for RawConsole <'a> {

	fn message_format (
		& mut self,
		message_arguments: fmt::Arguments,
	) {

		if self.status.is_some () {

			io::stderr ().write_fmt (
				format_args! (
					"\r{}{}{}{}\r\n{}\r\n",
					termion::cursor::Up (1),
					termion::clear::CurrentLine,
					message_arguments,
					self.status.as_ref ().unwrap (),
					self.status_suffix.as_ref ().unwrap_or (& "".to_string ()),
				),
			).unwrap_or_else (
				|error|

				(self.error_handler) (
					error)

			);

		} else {

			io::stderr ().write_fmt (
				format_args! (
					"{}\r\n",
					message_arguments),
			).unwrap_or_else (
				|error|

				(self.error_handler) (
					error)

			);

		}

	}

	fn status_format (
		& mut self,
		status_arguments: fmt::Arguments,
	) {

		let status =
			format! (
				"{}",
				status_arguments);

		let status: String =
			status.chars ().take (
				self.columns as usize,
			).collect ();

		if self.status.is_some () {

			io::stderr ().write_fmt (
				format_args! (
					"\r{}{}{}\r\n",
					termion::cursor::Up (1),
					termion::clear::CurrentLine,
					status),
			).unwrap_or_else (
				|error|

				(self.error_handler) (
					error)

			);

		} else {

			io::stderr ().write_fmt (
				format_args! (
					"{}\r\n",
					status),
			).unwrap_or_else (
				|error|

				(self.error_handler) (
					error)

			);

		}

		self.status =
			Some (status);

	}

	fn clear_status (
		& mut self,
	) {

		if self.status.is_some () {

			io::stderr ().write_fmt (
				format_args! (
					"\r{}{}",
					termion::cursor::Up (1),
					termion::clear::CurrentLine),
			).unwrap_or_else (
				|error|

				(self.error_handler) (
					error)

			);

		}

		self.status = None;
		self.status_suffix = None;

	}

	fn status_progress (
		& mut self,
		numerator: u64,
		denominator: u64,
	) {

		if self.status.is_none () {

			panic! (
				"Called status_progress () with no status");

		}

		self.status_suffix =
			Some (
				format! (
					" {}%",
					numerator * 100 / denominator));

		io::stderr ().write_fmt (
			format_args! (
				"\r{}{}{}{}\r\n",
				termion::cursor::Up (1),
				termion::clear::CurrentLine,
				self.status.as_ref ().unwrap (),
				self.status_suffix.as_ref ().unwrap_or (& "".to_string ()),
			),
		).unwrap_or_else (
			|error|

			(self.error_handler) (
				error)

		);

	}

	fn status_tick (
		& mut self,
	) {

		if self.status.is_none () {

			panic! (
				"Called status_progress () with no status");

		}

		self.status_suffix =
			Some (
				format! (
					" {}",
					self.status_tick_sequence [
						self.status_tick as usize]));

		io::stderr ().write_fmt (
			format_args! (
				"\r{}{}{}{}\r\n",
				termion::cursor::Up (1),
				termion::clear::CurrentLine,
				self.status.as_ref ().unwrap (),
				self.status_suffix.as_ref ().unwrap_or (& "".to_string ()),
			),
		).unwrap_or_else (
			|error|

			(self.error_handler) (
				error)

		);

		self.status_tick = (
			self.status_tick + 1
		) % self.status_tick_sequence.len () as u64;

	}

	fn status_done (
		& mut self,
	) {

		if self.status.is_none () {

			panic! (
				"Called status_done () with no status");

		}

		io::stderr ().write_fmt (
			format_args! (
				"\r{}{}{} done\r\n",
				termion::cursor::Up (1),
				termion::clear::CurrentLine,
				self.status.as_ref ().unwrap ()),
			).unwrap_or_else (
				|error|

				(self.error_handler) (
					error)

			);

		self.status =
			None;

	}

}

// ex: noet ts=4 filetype=rust
