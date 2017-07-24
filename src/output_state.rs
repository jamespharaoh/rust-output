use std::mem;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Weak;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use backend::*;
use output_log::*;

pub struct OutputState {

	backend: Option <Box <Backend>>,
	synchronous: bool,

	logs: Vec <OutputLogInternal>,
	next_log_id: u64,

	background_join_handle: Option <thread::JoinHandle <()>>,
	background_sender: Option <mpsc::Sender <()>>,

	paused: bool,
	changed: bool,

}

impl OutputState {

	pub fn new (
		backend: Option <Box <Backend>>,
		update_duration: Duration,
	) -> Arc <Mutex <OutputState>> {

		let synchronous =
			backend.as_ref ().map (
				|backend| backend.synchronous (),
			).unwrap_or (true);

		let real_self = OutputState {

			backend: backend,
			synchronous: synchronous,

			logs: Vec::new (),
			next_log_id: 0,

			background_join_handle: None,
			background_sender: None,

			paused: false,
			changed: false,

		};

		let shared_self =
			Arc::new (Mutex::new (
				real_self,
			));

		if ! synchronous {

			let mut real_self =
				shared_self.lock ().expect (
					"OutputState::new");

			let (background_sender, background_receiver) =
				mpsc::channel ();

			real_self.background_sender =
				Some (background_sender);

			{

				let shared_self =
					Arc::downgrade (
						& shared_self);

				real_self.background_join_handle = Some (
					thread::spawn (move ||
						Self::background_thread (
							shared_self,
							background_receiver,
							update_duration,
						)
					)
				);

			}

		}

		shared_self

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

		self.update_backend_auto (
			state);

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

	pub fn update_backend_auto (
		& mut self,
		state: OutputLogState,
	) {

		if state == OutputLogState::Message
		|| state == OutputLogState::Complete
		|| state == OutputLogState::Incomplete {

			self.update_backend_synchronous ();

		} else {

			self.update_backend_asynchronous ();

		};

	}

	pub fn update_backend_asynchronous (
		& mut self,
	) {

		if self.synchronous {
			self.update_backend_synchronous ();
		} else {
			self.changed = true;
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

		self.update_backend_real ();

	}

	pub fn flush (
		& mut self,
	) {

		let old_paused = self.paused;
		self.paused = false;

		self.update_backend_real ();

		self.paused = old_paused;

	}

	fn update_backend_synchronous (
		& mut self,
	) {

		self.changed = true;

		self.update_backend_real ();

	}

	fn update_backend_real (
		& mut self,
	) {

		if ! self.changed || self.paused {
			return;
		}

		if let Some (ref mut backend) =
			self.backend {

			backend.update (
				& self.logs);

		}

		let logs_temp =
			mem::replace (
				& mut self.logs,
				vec! []);

		self.logs =
			logs_temp.into_iter ().filter (
				|log_internal|
				log_internal.state () == OutputLogState::Running
			).collect ();

		self.changed = false;

	}

	fn background_thread (
		shared_state: Weak <Mutex <OutputState>>,
		background_receiver: mpsc::Receiver <()>,
		update_time: Duration,
	) {

		loop {

			// wait a bit

			match background_receiver.recv_timeout (
				update_time) {

				Ok (()) => (),

				Err (mpsc::RecvTimeoutError::Timeout) => (),

				Err (mpsc::RecvTimeoutError::Disconnected) => break,

			}

			// perform updates

			if let Some (ref mut shared_state) =
				shared_state.upgrade () {

				let mut state =
					shared_state.lock ().expect (
						"OutputState::background_thread");

				state.update_backend_real ();

			}

		}

	}

}

impl Drop for OutputState {

	fn drop (
		& mut self,
	) {

		// ask background thread to stop

		let background_sender =
			self.background_sender.take ().expect (
				"OutputState::drop");

		drop (background_sender);

		// wait for background thread to stop

		let background_join_handle =
			self.background_join_handle.take ().expect (
				"OutputState::drop");

		background_join_handle.join ().expect (
			"OutputState::drop");

		// perform final update

		self.paused = false;

		self.update_backend_real ();

	}

}

// ex: noet ts=4 filetype=rust
