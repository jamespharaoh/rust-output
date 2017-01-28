use std::sync::Arc;
use std::sync::Mutex;

use output_state::*;

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

impl OutputLog {

	pub fn new (
		output_state: Option <Arc <Mutex <OutputState>>>,
		log_id: u64,
	) -> OutputLog {

		OutputLog {
			output_state: output_state,
			log_id: log_id,
		}

	}

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

			}

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

	#[ inline ]
	pub fn new (
		log_id: u64,
		message: String,
		state: OutputLogState,
	) -> OutputLogInternal {

		OutputLogInternal {
			log_id: log_id,
			message: message.clone (),
			numerator: 0,
			denominator: 0,
			tick: 0,
			state: state,
		}

	}

	#[ inline ]
	pub fn log_id (& self) -> u64 {
		self.log_id
	}

	#[ inline ]
	pub fn message (& self) -> & str {
		& self.message
	}

	#[ inline ]
	pub fn state (& self) -> OutputLogState {
		self.state
	}

	#[ inline ]
	pub fn numerator (& self) -> u64 {
		self.numerator
	}

	#[ inline ]
	pub fn denominator (& self) -> u64 {
		self.denominator
	}

	#[ inline ]
	pub fn tick (& self) -> u64 {
		self.tick
	}

}

// ex: noet ts=4 filetype=rust
