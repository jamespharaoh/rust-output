use std::cmp;
use std::io;
use std::io::Stdout;

use termion;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

use output::Output;

pub struct RawConsole {
	terminal: Option <RawTerminal <Stdout>>,
	columns: u64,
	status: Option <String>,
}

impl RawConsole {

	pub fn new (
	) -> Option <RawConsole> {

		let mut terminal =
			match io::stdout ().into_raw_mode () {

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

		Some (
			RawConsole {
				terminal: Some (terminal),
				columns: columns,
				status: None,
			}
		)

	}

}

impl Output for RawConsole {

	fn status (
		& mut self,
		status: & str,
	) {

		let status =
			& status [
				0 .. cmp::min (
					self.columns as usize,
					status.len (),
				)
			];

		if self.status.is_some () {

			stderr! (
				"\r\x1b[A\x1b[K{}\r\n",
				status);

		} else {

			stderr! (
				"{}\r\n",
				status);

		}

		self.status =
			Some (status.to_owned ());

	}

	fn message (
		& mut self,
		message: & str,
	) {

		if self.status.is_some () {

			stderr! (
				"\r\x1b[A\x1b[K{}\r\n{}\r\n",
				message,
				self.status.as_ref ().unwrap ());

		} else {

			stderr! (
				"{}\r\n",
				message);

		}

	}

	fn clear_status (
		& mut self,
	) {

		if self.status.is_some () {

			stderr! (
				"\r\x1b[A\x1b[K");

		}

		self.status =
			None;

	}

}

// ex: noet ts=4 filetype=rust
