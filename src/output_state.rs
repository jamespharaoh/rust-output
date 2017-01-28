use std::mem;

use backend::*;
use output_log::*;

pub struct OutputState {
	backend: Option <Box <Backend>>,
	logs: Vec <OutputLogInternal>,
	next_log_id: u64,
	paused: bool,
	changed: bool,
}

impl OutputState {

	pub fn new (
		backend: Option <Box <Backend>>,
	) -> OutputState {

		OutputState {
			backend: backend,
			logs: Vec::new (),
			next_log_id: 0,
			paused: false,
			changed: false,
		}

	}

	#[ inline ]
	pub fn add_log (
		& mut self,
		message: String,
		state: OutputLogState,
	) -> u64 {

		let log_id = self.next_log_id;
		self.next_log_id += 1;

		let log_internal =
			OutputLogInternal::new (
				log_id,
				message,
				state);

		self.logs.push (
			log_internal);

		self.update_backend ();

		log_id

	}

	pub fn get_log_internal (
		& mut self,
		log_id: u64,
	) -> Option <& mut OutputLogInternal> {

		self.logs.iter_mut ().filter (
			|log_internal|
			log_internal.log_id () == log_id
		).next ()

	}

	pub fn update_backend (
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
					|log_internal|
					log_internal.state () != OutputLogState::Running
				).collect ();

		}

	}

	pub fn pause (
		& mut self,
	) {

		self.paused = true;

	}

	pub fn unpause (
		& mut self,
	) {

		self.paused = false;

		if self.changed {
			self.update_backend ()
		}

	}

}

// ex: noet ts=4 filetype=rust
