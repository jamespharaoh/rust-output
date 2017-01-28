use std::mem;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Weak;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use backend::*;
use output_log::*;

pub struct OutputState {

	backend: Option <Box <Backend>>,

	logs: Vec <OutputLogInternal>,
	next_log_id: u64,

	background_join_handle: Option <thread::JoinHandle <()>>,
	background_sender: Option <mpsc::Sender <Event>>,

	paused: bool,
	changed: bool,

}

enum Event {
	Changed,
	Stop,
}

impl OutputState {

	pub fn new (
		backend: Option <Box <Backend>>,
		update_duration: Duration,
	) -> Arc <Mutex <OutputState>> {

		let real_self = OutputState {

			backend: backend,

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

		{

			let mut real_self =
				shared_self.lock ().unwrap ();

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

		if ! self.changed {

			self.changed = true;

			if ! self.paused {

				self.background_sender.as_ref ().expect (
					"Output state sender disappeared",
				).send (
					Event::Changed,
				).expect (
					"Output state sender disconnected");

			}

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

		if self.paused {

			self.paused = false;

			if self.changed {

				self.background_sender.as_ref ().expect (
					"Output state sender disappeared",
				).send (
					Event::Changed,
				).expect (
					"Output state sender disconnected",
				);

			}

		}

	}

	fn update_backend_real (
		& mut self,
	) {

		if ! self.changed {
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
			logs_temp.into_iter ().skip_while (
				|log_internal|
				log_internal.state () != OutputLogState::Running
			).collect ();

		self.changed = false;

	}

	fn background_thread (
		shared_state: Weak <Mutex <OutputState>>,
		background_receiver: mpsc::Receiver <Event>,
		update_time: Duration,
	) {

		let mut next_update =
			Instant::now ();

		let mut changes = false;

		loop {

			// wait for event or delayed update

			let event = if changes {

				let now = Instant::now ();

				if next_update <= now {
					Event::Changed
				} else {

					match background_receiver.recv_timeout (
						next_update - now,
					) {

						Ok (event) =>
							event,

						Err (mpsc::RecvTimeoutError::Timeout) =>
							Event::Changed,

						Err (mpsc::RecvTimeoutError::Disconnected) =>
							panic! (
								"Output state receiver disconnected"),

					}

				}

			} else {

				match background_receiver.recv () {

					Ok (event) =>
						event,

					Err (mpsc::RecvError) =>
						panic! (
							"Output state receiver disconnected"),

				}

			};

			// process event

			match event {
				Event::Changed => changes = true,
				Event::Stop => break,
			};

			// perform update when appropriate

			if changes && next_update <= Instant::now () {

				if let Some (ref mut shared_state) =
					shared_state.upgrade () {
					
					let mut state =
						shared_state.lock ().unwrap ();

					state.update_backend_real ();

					next_update = Instant::now () + update_time;

					changes = false;

				} else {
					break;
				}

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
			self.background_sender.take ().unwrap ();

		background_sender.send (
			Event::Stop,
		).unwrap ();

		// wait for background thread to stop

		let background_join_handle =
			self.background_join_handle.take ().unwrap ();

		background_join_handle.join ().unwrap ();

		// perform final update

		self.update_backend_real ();

	}

}

// ex: noet ts=4 filetype=rust
