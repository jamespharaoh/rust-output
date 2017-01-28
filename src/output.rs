use std::fmt;
use std::sync::Arc;
use std::sync::Mutex;

use backend::*;
use output_log::*;
use output_state::*;

#[ derive (Clone) ]
pub struct Output {
	state: Arc <Mutex <OutputState>>,
}

impl Output {

	pub fn new (
		backend: Option <Box <Backend>>,
	) -> Output {

		Output {
			state: Arc::new (Mutex::new (
				OutputState::new  (
					backend),
			)),
		}

	}

	#[ inline ]
	pub fn message_format (
		& self,
		arguments: fmt::Arguments,
	) {

		self.add_log (
			format! (
				"{}",
				arguments),
			OutputLogState::Message);

	}

	#[ inline ]
	pub fn message <
		Message: Into <String>,
	> (
		& self,
		message: Message,
	) {

		self.add_log (
			message.into (),
			OutputLogState::Message);

	}

	pub fn start_job <
		MessageString: Into <String>,
	> (
		& self,
		message: MessageString,
	) -> OutputLog {

		self.add_log (
			message.into (),
			OutputLogState::Running)

	}

	pub fn pause (
		& self,
	) {

		let mut self_state =
			self.state.lock ().unwrap ();

		self_state.pause ();

	}

	pub fn unpause (
		& self,
	) {

		let mut self_state =
			self.state.lock ().unwrap ();

		self_state.unpause ();

	}

	pub fn add_log (
		& self,
		message: String,
		state: OutputLogState,
	) -> OutputLog {

		let log_id = {

			let mut self_state =
				self.state.lock ().unwrap ();

			self_state.add_log (
				message,
				state)

		};

		OutputLog::new (
			Some (self.state.clone ()),
			log_id,
		)

	}

}

// ex: noet ts=4 filetype=rust
