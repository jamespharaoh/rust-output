lazy_static! {

	pub static ref DEFAULT: Vec <String> = vec! [
		"|          |",
		"|#         |",
		"|##        |",
		"|###       |",
		"|####      |",
		"|#####     |",
		"|######    |",
		"| ######   |",
		"|  ######  |",
		"|   ###### |",
		"|    ######|",
		"|     #####|",
		"|      ####|",
		"|        ##|",
		"|         #|",
	].iter ().map (|string| string.to_string ()).collect ();

}

// ex: noet ts=4 filetype=rust
