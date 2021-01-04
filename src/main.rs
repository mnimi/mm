//! A Minecraft clone written in Rust.
#![allow(overflowing_literals)]

// Vulkan API wrapper for the Rust programming language
extern crate ash;

// Better error handling.
extern crate anyhow;

// Lazily initialised statics.
#[macro_use] extern crate lazy_static;

// Logging.
#[macro_use] extern crate log;

// Data de/serialization.
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate toml;

// Asynchronous input/output.
extern crate tokio;
use tokio::runtime::Runtime;

pub use log::Level as LogLevel;

pub mod data;
pub mod gfx;

lazy_static!
{
  pub static ref ASYNC: tokio::runtime::Runtime = Runtime::new().unwrap();
}

fn main() -> Result<(), anyhow::Error>
{
  use self::data::App;
  let mut app = App::init()?;

  log!(LogLevel::Info, "Starting...");

  // Call async functions inside of the async runtime.
  ASYNC.block_on(async {
    app.execute().await;
  });



  Ok(())
}
