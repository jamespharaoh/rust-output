use std::cell::RefCell;
use std::fmt;

use backend::*;

pub struct Output {
	backend: RefCell <BoxBackend>,
}

impl Output {

	pub fn new (
		backend: BoxBackend,
	) -> Output {

		Output {

			backend:
				RefCell::new (
					backend),

		}

	}

	pub fn message <
		Message: Into <String>,
	> (
		& self,
		message: Message,
	) {

		let mut backend =
			self.backend.borrow_mut ();

		backend.message_format (
			format_args! (
				"{}",
				message.into ()))

	}

	pub fn message_format (
		& self,
		message_arguments: fmt::Arguments,
	) {

		let mut backend =
			self.backend.borrow_mut ();

		backend.message_format (
			message_arguments)

	}

	pub fn status <
		Status: Into <String>,
	> (
		& self,
		status: Status,
	) {

		let mut backend =
			self.backend.borrow_mut ();

		backend.status_format (
			format_args! (
				"{}",
				status.into ()))

	}

	pub fn status_format (
		& self,
		status_arguments: fmt::Arguments,
	) {

		let mut backend =
			self.backend.borrow_mut ();

		backend.status_format (
			status_arguments)

	}

	pub fn clear_status (
		& self,
	) {

		let mut backend =
			self.backend.borrow_mut ();

		backend.clear_status ()

	}

	pub fn status_progress (
		& self,
		numerator: u64,
		denominator: u64,
	) {

		let mut backend =
			self.backend.borrow_mut ();

		backend.status_progress (
			numerator,
			denominator)

	}

	pub fn status_tick (
		& self,
	) {

		let mut backend =
			self.backend.borrow_mut ();

		backend.status_tick ()

	}

	pub fn status_done (
		& self,
	) {

		let mut backend =
			self.backend.borrow_mut ();

		backend.status_done ()

	}

}

// ex: noet ts=4 filetype=rust
