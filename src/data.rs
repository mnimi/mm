use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::io::Write;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

use anyhow::Result;

pub mod block;
pub mod world;

pub const APP_DATA_FILE: &'static str = "app_data.toml";

/// Globally accessible, shared application data.
pub struct App
{
  pub data: Mutex<AppData>,
}

impl App
{
  /// Initialise the application.
  ///
  /// Returns a result containing one of two possible outcomes of the initialisation:
  /// Success, in which case we continue into the main application loop
  ///
  /// or
  ///
  /// Failure, where we quit and enter into the cleanup of our app's resources
  /// and close it.
  pub fn init() -> Result<Self>
  {
    use anyhow::Error;
    use crate::LogLevel;

    let data = Mutex::new(if std::fs::metadata(&APP_DATA_FILE).is_ok() {
      AppData::new()
          .load()
          .unwrap()
    } else {
      log!(LogLevel::Error, "File does not exist. Creating...");
      let dat = AppData::new();
      dat.save().unwrap();


      dat
    });


    Ok(App { data })
  }

  pub async fn execute(&self) -> Result<()>
  {
    Ok(())
  }

  pub fn run_time(&self)
  {
    todo!()
  }
}

/// Globally accessible application data.
#[derive(Debug, Deserialize, Serialize)]
pub struct AppData
{
  /// The reported time from the host machine.
  time: SystemTime,
  /// The chosen name of the player.
  pub username: String,
  /// A list of worlds.
  worlds: Vec<String>,
}

impl AppData
{
  pub fn new() -> Self
  {
    AppData {
      time: SystemTime::now(),
      username: "USERNAME".to_string(),
      worlds: vec![]
    }
  }

  pub fn save(&self) -> Result<()>
  {
    use std::io::Write;

    let mut fi = if std::fs::metadata(&APP_DATA_FILE).is_ok() {
      File::open(&APP_DATA_FILE)
    } else {
      File::create(&APP_DATA_FILE)
    }.unwrap();

    let content = toml::to_string(self).unwrap();

    fi.write_all(content.as_bytes()).unwrap();


    Ok(())
  }

  pub fn load(mut self) -> Result<Self>
  {
    use std::io::Read;
    use crate::LogLevel;

    let mut fi = if std::fs::metadata(&APP_DATA_FILE).is_ok() {
      File::open(&APP_DATA_FILE)
    } else {
      log!(LogLevel::Error, "Configuration file does not exist!");
      log!(LogLevel::Info, "Creating config...");
      &self.save().unwrap();


      Err(Error::from_raw_os_error(1))
    }.unwrap();

    let mut content = String::new();
    fi.read_to_string(&mut content).unwrap();
    self = toml::from_str(content.as_str()).unwrap();


    Ok(self)
  }
}


