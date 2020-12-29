use anyhow::{Result, Error};
//mutex is sync thing so we can play around with it but nothing else
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

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
  pub async fn init() -> Result<Self>
  {



    Ok(App {
      data: Mutex::new(AppData::new())
    })
  }

  pub fn run_time(&self)
  {
    todo!()
  }
}

/// Globally accessible application data.
#[derive(Debug, Serialize, Deserialize)]
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
}
