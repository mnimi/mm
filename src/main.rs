//! A Minecraft clone written in Rust.

// Better error handling.
extern crate anyhow;

// Logging.
#[macro_use] extern crate log;

// Data de/serialization.
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate toml;

// Asynchronous input/output.
extern crate tokio;

pub use log::Level as LogLevel;

pub mod data;

fn main() -> Result<(), anyhow::Error>
{
  use self::data::App;
  let mut app = App::init()?;

  log!(LogLevel::Info, "Starting...");


  Ok(())
}
