extern crate libc;
extern crate termion;

#[ macro_use ]
pub mod stderr;

mod output;
mod pipe;
mod rawconsole;

use std::fs::File;

pub use output::*;
pub use pipe::*;
pub use rawconsole::*;

pub fn open (
) -> Box <Output> {

	if let Ok (stderr) =
		File::open ("/dev/stderr") {

		if termion::is_tty (& stderr) {

			return Box::new (
				RawConsole::new ().unwrap ())

		}

	}

	Box::new (
		PipeOutput::new ())

}

// ex: noet ts=4 filetype=rust
