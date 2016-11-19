extern crate termion;

#[ macro_use ]
pub mod stderr;

mod output;
mod rawconsole;

pub use output::Output;
pub use rawconsole::RawConsole;

// ex: noet ts=4 filetype=rust
