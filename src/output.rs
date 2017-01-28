use std::fmt;
use std::mem;
use std::sync::Arc;
use std::sync::Mutex;

use backend::*;

#[ derive (Clone) ]
pub struct Output {
	state: Arc <Mutex <OutputState>>,
}

pub struct OutputState {
	backend: Option <Box <Backend>>,
	logs: Vec <OutputLogInternal>,
	next_log_id: u64,
	paused: bool,
	changed: bool,
}

pub struct OutputLog {
	output_state: Option <Arc <Mutex <OutputState>>>,
	log_id: u64,
}

pub type OutputJob = OutputLog;

#[ derive (Clone, Copy, PartialEq) ]
pub enum OutputLogState {
	Message,
	Running,
	Complete,
	Incomplete,
	Removed,
}

pub struct OutputLogInternal {
	log_id: u64,
	message: String,
	numerator: u64,
	denominator: u64,
	tick: u64,
	state: OutputLogState,
}

impl Output {

	pub fn new (
		backend: Option <Box <Backend>>,
	) -> Output {

		Output {
			state: Arc::new (Mutex::new (OutputState {
				backend: backend,
				logs: Vec::new (),
				next_log_id: 0,
				paused: false,
				changed: false,
			})),
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

	fn add_log (
		& self,
		message: String,
		state: OutputLogState,
	) -> OutputLog {

		let mut self_state =
			self.state.lock ().unwrap ();

		let log_id = self_state.next_log_id;
		self_state.next_log_id += 1;

		let log_internal =
			OutputLogInternal {
				log_id: log_id,
				message: message.clone (),
				numerator: 0,
				denominator: 0,
				tick: 0,
				state: state,
			};

		self_state.logs.push (
			log_internal);

		let log =
			OutputLog {
				output_state: Some (self.state.clone ()),
				log_id: log_id,
			};

		self_state.update_backend ();

		log

	}

	pub fn pause (
		& self,
	) {

		let mut self_state =
			self.state.lock ().unwrap ();

		self_state.paused = true;

	}

	pub fn unpause (
		& self,
	) {

		let mut self_state =
			self.state.lock ().unwrap ();

		self_state.paused = false;

		if self_state.changed {
			self_state.update_backend ()
		}

	}

}

impl OutputState {

	fn get_log_internal (
		& mut self,
		log_id: u64,
	) -> Option <& mut OutputLogInternal> {

		self.logs.iter_mut ().filter (
			|log_internal| log_internal.log_id == log_id
		).next ()

	}

	fn update_backend (
		& mut self,
	) {

		if self.paused {

			self.changed = true;

		} else {

			self.changed = false;

			if let Some (ref mut backend) = self.backend {

				backend.update (
					& self.logs);

			}

			let logs_temp =
				mem::replace (
					& mut self.logs,
					vec! []);

			self.logs =
				logs_temp.into_iter ().skip_while (
					|log_internal| log_internal.state != OutputLogState::Running
				).collect ();

		}

	}

}

impl OutputLog {

	pub fn null (
	) -> OutputLog {

		OutputLog {
			output_state: None,
			log_id: 0,
		}

	}

	pub fn progress (
		& self,
		numerator: u64,
		denominator: u64,
	) {

		if let Some (ref output_state) =
			self.output_state {

			let mut output_state =
				output_state.lock ().unwrap ();

			{

				let log_internal =
					output_state.get_log_internal (
						self.log_id,
					).unwrap ();

				if log_internal.state != OutputLogState::Running {
					panic! ();
				}

				log_internal.numerator = numerator;
				log_internal.denominator = denominator;

			}

			output_state.update_backend ();

		}

	}

	pub fn tick (
		& self,
	) {

		if let Some (ref output_state) =
			self.output_state {

			let mut output_state =
				output_state.lock ().unwrap ();

			{

				let log_internal =
					output_state.get_log_internal (
						self.log_id,
					).unwrap ();

				if log_internal.state != OutputLogState::Running {
					panic! ();
				}

				log_internal.tick += 1;

			}

			output_state.update_backend ();

		}

	}

	pub fn remove (
		self,
	) {

		if let Some (ref output_state) =
			self.output_state {

			let mut output_state =
				output_state.lock ().unwrap ();

			{

				let log_internal =
					output_state.get_log_internal (
						self.log_id,
					).unwrap ();

				if log_internal.state != OutputLogState::Running {
					panic! ();
				}

				log_internal.state = OutputLogState::Removed;

			}

			output_state.update_backend ();

		}

	}

	pub fn complete (
		self,
	) {

		if let Some (ref output_state) =
			self.output_state {

			let mut output_state =
				output_state.lock ().unwrap ();

			{

				let log_internal =
					output_state.get_log_internal (
						self.log_id,
					).unwrap ();

				if log_internal.state != OutputLogState::Running {
					panic! ();
				}

				log_internal.state = OutputLogState::Complete;

			};

			output_state.update_backend ();

		}

	}

	pub fn incomplete (
		self,
	) {

		if let Some (ref output_state) =
			self.output_state {

			let mut output_state =
				output_state.lock ().unwrap ();

			{

				let log_internal =
					output_state.get_log_internal (
						self.log_id,
					).unwrap ();

				if log_internal.state != OutputLogState::Running {
					panic! ();
				}

				log_internal.state = OutputLogState::Incomplete;

			}

			output_state.update_backend ();

		}

	}

	pub fn update (
		& self,
		message: String,
	) {

		if let Some (ref output_state) =
			self.output_state {

			let mut output_state =
				output_state.lock ().unwrap ();

			{

				let log_internal =
					output_state.get_log_internal (
						self.log_id,
					).unwrap ();

				if log_internal.state != OutputLogState::Running {
					panic! ();
				}

				log_internal.message = message;

			}

			output_state.update_backend ();

		}

	}

	pub fn replace (
		self,
		message: String,
	) {

		if let Some (ref output_state) =
			self.output_state {

			let mut output_state =
				output_state.lock ().unwrap ();

			{

				let log_internal =
					output_state.get_log_internal (
						self.log_id,
					).unwrap ();

				if log_internal.state != OutputLogState::Running
				&& log_internal.state != OutputLogState::Message {
					panic! ();
				}

				log_internal.state = OutputLogState::Message;
				log_internal.message = message;

			}

			output_state.update_backend ();

		}

	}

}

impl Drop for OutputLog {

	fn drop (
		& mut self,
	) {

		if let Some (ref output_state) =
			self.output_state {

			let mut output_state =
				output_state.lock ().unwrap ();

			{

				if let Some (log_internal) =
					output_state.get_log_internal (
						self.log_id,
					) {

					if log_internal.state == OutputLogState::Running {
						log_internal.state = OutputLogState::Incomplete;
					}

				}

			};

			output_state.update_backend ();

		}

	}

}

impl OutputLogInternal {

	pub fn message (& self) -> & str {
		& self.message
	}

	pub fn state (& self) -> OutputLogState {
		self.state
	}

	pub fn numerator (& self) -> u64 {
		self.numerator
	}

	pub fn denominator (& self) -> u64 {
		self.denominator
	}

	pub fn tick (& self) -> u64 {
		self.tick
	}

}

// ex: noet ts=4 filetype=rust
