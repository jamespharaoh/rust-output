use std::fmt;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use backend::*;
use output_log::*;
use output_state::*;

#[ derive (Clone) ]
pub struct Output {
	state: Arc <Mutex <OutputState>>,
	prefix: String,
	notice: bool,
	debug: bool,
}

impl Output {

	#[ inline ]
	pub fn new (
		backend: Option <Box <Backend>>,
	) -> Output {

		Output {
			state: OutputState::new (
				backend,
				Duration::from_millis (100)),
			prefix: "".to_string (),
			notice: true,
			debug: false,
		}

	}

	#[ inline ]
	pub fn new_with_options (
		backend: Option <Box <Backend>>,
		prefix: String,
		notice: bool,
		debug: bool,
	) -> Output {

		Output {
			state: OutputState::new (
				backend,
				Duration::from_millis (100)),
			prefix: prefix,
			notice: notice,
			debug: debug,
		}

	}

	#[ inline ]
	pub fn disable_notices (
		& self,
	) -> Output {

		Output {
			state: self.state.clone (),
			prefix: self.prefix.clone (),
			notice: false,
			debug: false,
		}

	}

	#[ inline ]
	pub fn enable_notices (
		& self,
	) -> Output {

		Output {
			state: self.state.clone (),
			prefix: self.prefix.clone (),
			notice: true,
			debug: false,
		}

	}

	#[ inline ]
	pub fn enable_debug (
		& self,
	) -> Output {

		Output {
			state: self.state.clone (),
			prefix: self.prefix.clone (),
			notice: true,
			debug: true,
		}

	}

	#[ inline ]
	pub fn prefix (
		& self,
		prefix: String,
	) -> Output {

		Output {
			state: self.state.clone (),
			prefix: format! (
				"{}{}",
				self.prefix,
				prefix),
			notice: true,
			debug: true,
		}

	}

	#[ inline ]
	pub fn message_format (
		& self,
		arguments: fmt::Arguments,
	) {

		self.add_log (
			format! (
				"{}{}",
				self.prefix,
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
			format! (
				"{}{}",
				self.prefix,
				message.into ()),
			OutputLogState::Message);

	}

	#[ inline ]
	pub fn debug_format (
		& self,
		arguments: fmt::Arguments,
	) {

		if self.debug {

			self.add_log (
				format! (
					"{}{}",
					self.prefix,
					arguments),
				OutputLogState::Message);

		}

	}

	#[ inline ]
	pub fn notice <
		Message: Into <String>,
	> (
		& self,
		message: Message,
	) {

		if self.notice {

			self.add_log (
				format! (
					"{}{}",
					self.prefix,
					message.into ()),
				OutputLogState::Message);

		}

	}

	#[ inline ]
	pub fn notice_format (
		& self,
		arguments: fmt::Arguments,
	) {

		if self.notice {

			self.add_log (
				format! (
					"{}{}",
					self.prefix,
					arguments),
				OutputLogState::Message);

		}

	}

	#[ inline ]
	pub fn debug <
		Message: Into <String>,
	> (
		& self,
		message: Message,
	) {

		if self.debug {

			self.add_log (
				format! (
					"{}{}",
					self.prefix,
					message.into ()),
				OutputLogState::Message);

		}

	}

	#[ inline ]
	pub fn start_job <
		MessageString: Into <String>,
	> (
		& self,
		message: MessageString,
	) -> OutputLog {

		self.add_log (
			format! (
				"{}{}",
				self.prefix,
				message.into ()),
			OutputLogState::Running)

	}

	#[ inline ]
	pub fn pause (
		& self,
	) {

		let mut self_state =
			self.state.lock ().unwrap ();

		self_state.pause ();

	}

	#[ inline ]
	pub fn unpause (
		& self,
	) {

		let mut self_state =
			self.state.lock ().unwrap ();

		self_state.unpause ();

	}

	#[ inline ]
	pub fn flush (
		& self,
	) {

		let mut self_state =
			self.state.lock ().unwrap ();

		self_state.flush ();

	}

	#[ inline ]
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
