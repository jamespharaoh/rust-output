use std::cmp;
use std::io;
use std::io::Stdout;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;

use libc;

use output::Output;

use termion;
use termion::AsyncReader;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

pub struct RawConsole {

	output: Option <RawTerminal <Stdout>>,
	columns: u64,
	status: Option <String>,

	input_receiver: Receiver <Key>,
	input_thread: JoinHandle <()>,

}

impl RawConsole {

	pub fn new (
	) -> Option <RawConsole> {

		// setup output

		let mut output =
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

		// setup input

		let (input_sender, input_receiver) =
			mpsc::channel ();

		let input_thread =
			thread::spawn (
				move || {

			Self::input_thread (
				input_sender);

		});

		Some (
			RawConsole {

				output: Some (output),
				columns: columns,
				status: None,

				input_receiver: input_receiver,
				input_thread: input_thread,

			}
		)

	}

	fn input_thread (
		sender: Sender <Key>,
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


impl Output for RawConsole {

	fn status (
		& mut self,
		status: & str,
	) {

		let status: String =
			status.chars ().take (
				self.columns as usize,
			).collect ();

		if self.status.is_some () {

			stderr! (
				"\r{}{}{}\r\n",
				termion::cursor::Up (1),
				termion::clear::CurrentLine,
				status);

		} else {

			stderr! (
				"{}\r\n",
				status);

		}

		self.status =
			Some (status);

	}

	fn message (
		& mut self,
		message: & str,
	) {

		if self.status.is_some () {

			stderr! (
				"\r{}{}{}\r\n{}\r\n",
				termion::cursor::Up (1),
				termion::clear::CurrentLine,
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
				"\r{}{}",
				termion::cursor::Up (1),
				termion::clear::CurrentLine);

		}

		self.status =
			None;

	}

}

// ex: noet ts=4 filetype=rust
