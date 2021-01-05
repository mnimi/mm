//! A Minecraft clone written in Rust.
#![allow(overflowing_literals)]

// Better error handling.
extern crate anyhow;

// Vulkan API wrapper for the Rust programming language
extern crate erupt;

// Lazily initialised statics.
#[macro_use]
extern crate lazy_static;

// Logging.
#[macro_use]
extern crate log;

// Data de/serialization.
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

// Asynchronous input/output.
extern crate tokio;
use tokio::runtime::Runtime;

pub use log::Level as LogLevel;

pub mod data;
pub mod gfx;

lazy_static! {
  pub static ref ASYNC: tokio::runtime::Runtime = Runtime::new().unwrap();
}

fn main() -> Result<(), anyhow::Error>
{
  use self::data::APPLICATION;

  log!(LogLevel::Info, "Starting...");

  // Call async functions inside of the async runtime block.
  ASYNC.block_on(async {
    &APPLICATION.exec_setup().await.unwrap();
  });

  Ok(())
}
