//! A simple Minecraft clone written in Rust.
#![deny(clippy::all)]
#![deny(missing_docs)]
#![allow(overflowing_literals)]
#![allow(dead_code)]

// Better error handling.
extern crate anyhow;

// Vulkan API wrapper for the Rust programming language.
extern crate erupt;

// Lazily initialised statics.
#[macro_use]
extern crate lazy_static;

// Logging.
#[macro_use]
extern crate log;

// Data de/serialization.
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
  /// Async runtime provided by tokio.
  pub static ref ASYNC: tokio::runtime::Runtime = Runtime::new().unwrap();
}

/// Application entry point.
fn main() -> Result<(), anyhow::Error>
{
  log!(LogLevel::Info, "Starting...");

  // Call async functions inside of the async runtime block.
  &ASYNC.block_on(async {});

  Ok(())
}
