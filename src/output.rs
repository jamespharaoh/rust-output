use std::fmt;
use std::sync::Arc;
use std::sync::Mutex;

use backend::*;

#[ derive (Clone) ]
pub struct Output {
	backend: Option <Arc <Mutex <BoxBackend>>>,
}

impl Output {

	#[ inline ]
	pub fn new (
		backend: Option <BoxBackend>,
	) -> Output {

		Output {

			backend: match backend {

				Some (backend) =>
					Some (Arc::new (Mutex::new (
						backend
					))),

				None =>
					None,

			},

		}

	}

	#[ inline ]
	pub fn message <
		Message: Into <String>,
	> (
		& self,
		message: Message,
	) {

		if let Some (ref backend) = self.backend {

			let mut backend =
				backend.lock ().unwrap ();

			backend.message_format (
				format_args! (
					"{}",
					message.into ()))

		}

	}

	#[ inline ]
	pub fn message_format (
		& self,
		message_arguments: fmt::Arguments,
	) {

		if let Some (ref backend) = self.backend {

			let mut backend =
				backend.lock ().unwrap ();

			backend.message_format (
				message_arguments)

		}

	}

	#[ inline ]
	pub fn status <
		Status: Into <String>,
	> (
		& self,
		status: Status,
	) {

		if let Some (ref backend) = self.backend {

			let mut backend =
				backend.lock ().unwrap ();

			backend.status_format (
				format_args! (
					"{}",
					status.into ()))

		}

	}

	#[ inline ]
	pub fn status_format (
		& self,
		status_arguments: fmt::Arguments,
	) {

		if let Some (ref backend) = self.backend {

			let mut backend =
				backend.lock ().unwrap ();


			backend.status_format (
				status_arguments)

		}

	}

	#[ inline ]
	pub fn clear_status (
		& self,
	) {

		if let Some (ref backend) = self.backend {

			let mut backend =
				backend.lock ().unwrap ();

			backend.clear_status ()

		}

	}

	#[ inline ]
	pub fn status_progress (
		& self,
		numerator: u64,
		denominator: u64,
	) {

		if let Some (ref backend) = self.backend {

			let mut backend =
				backend.lock ().unwrap ();

			backend.status_progress (
				numerator,
				denominator)

		}

	}

	#[ inline ]
	pub fn status_tick (
		& self,
	) {

		if let Some (ref backend) = self.backend {

			let mut backend =
				backend.lock ().unwrap ();

			backend.status_tick ()

		}

	}

	#[ inline ]
	pub fn status_done (
		& self,
	) {

		if let Some (ref backend) = self.backend {

			let mut backend =
				backend.lock ().unwrap ();

			backend.status_done ()

		}

	}

}

// ex: noet ts=4 filetype=rust
