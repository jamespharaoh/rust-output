//! Simple tool for user-friendly output in Rust CLI programs.
//!
//! The main goal is to enable programs to provide feedback about their
//! progress to users, in a more advanced way than outputting lines. This
//! library concerns itself with stderr and lets stdout be used for direct
//! output.
//!
//! The central concept is that a message can be logged "permanently" or
//! "temporarily". Temporary messages will be removed or replaced, whereas
//! permament messages will remain on screen. Permanent messages will appear
//! above any temporary message which is currently present.
//!
//! To use, add it to your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! output = "*"
//! ```

#![ allow (unused_parens) ]

#![ deny (non_camel_case_types) ]
#![ deny (non_snake_case) ]
#![ deny (non_upper_case_globals) ]
#![ deny (unreachable_patterns) ]
#![ deny (unused_comparisons) ]
#![ deny (unused_must_use) ]

#[ macro_use ]
extern crate lazy_static;

extern crate libc;
extern crate termion;

mod backend;
mod console;
mod output;
mod output_log;
mod output_state;
mod pipe;
mod rawconsole;
mod ticksequence;

use std::fs::File;

pub use backend::*;
pub use console::*;
pub use output::*;
pub use output_log::*;
pub use pipe::*;
pub use rawconsole::*;

pub fn open (
) -> Output {

	Output::new (Some (

		open_backend (
			false)

	))

}

pub fn open_raw (
) -> Output {

	Output::new (Some (

		open_backend (
			true)

	))

}

pub fn pipe (
) -> Output {

	let error_handler =
		Box::new (
			|_error| ()
		);

	Output::new (Some (Box::new (

		PipeOutput::new (
			error_handler))

	))

}

pub fn null (
) -> Output {

	Output::new (None)

}

pub fn open_backend (
	raw: bool,
) -> BoxBackend {

	let error_handler =
		Box::new (
			|_error| ()
		);

	let stdin_is_tty =
		is_tty ("/dev/stdin");

	let stdout_is_tty =
		is_tty ("/dev/stdout");

	let stderr_is_tty =
		is_tty ("/dev/stderr");

	if stdin_is_tty && stdout_is_tty && stderr_is_tty && raw {

		Box::new (
			RawConsole::new (
				error_handler,
				& ticksequence::DEFAULT,
			).unwrap ()
		)

	} else if stderr_is_tty {

		Box::new (
			Console::new (
				error_handler,
				& ticksequence::DEFAULT))

	} else {

		Box::new (
			PipeOutput::new (
				error_handler))

	}

}

fn is_tty (
	path: & str,
) -> bool {

	if let Ok (file) =
		File::open (
			path) {

		termion::is_tty (
			& file)

	} else {

		false

	}

}

#[ macro_export ]
macro_rules! output_message (

	(
		$ output : expr ,
		$ ( $ argument : tt ) *
	) => {
		$ output.message_format (
			format_args! (
				$ ( $ argument ) *
			)
		)
	};

);

#[ macro_export ]
macro_rules! output_notice (

	(
		$ output : expr ,
		$ ( $ argument : tt ) *
	) => {
		$ output.notice_format (
			format_args! (
				$ ( $ argument ) *
			)
		)
	};

);

#[ macro_export ]
macro_rules! output_debug (

	(
		$ output : expr ,
		$ ( $ argument : tt ) *
	) => {
		$ output.debug_format (
			format_args! (
				$ ( $ argument ) *
			)
		)
	};

);

#[ macro_export ]
macro_rules! output_job_start (

	(
		$ output : expr ,
		$ ( $ argument : tt ) *
	) => {
		$ output.start_job (
			format! (
				$ ( $ argument ) *
			)
		)
	};

);

#[ macro_export ]
macro_rules! output_job_update (

	(
		$ output_job : expr ,
		$ ( $ argument : tt ) *
	) => {
		$ output_job.update (
			format! (
				$ ( $ argument ) *
			)
		)
	};

);

#[ macro_export ]
macro_rules! output_job_replace (

	(
		$ output_job : expr ,
		$ ( $ argument : tt ) *
	) => {
		$ output_job.replace (
			format! (
				$ ( $ argument ) *
			)
		)
	};

);

// ex: noet ts=4 filetype=rust
