/*!
General app data and game data.
*/

use anyhow::Result;
use std::fs::File;
use std::io::Error;
use std::sync::Mutex;
use std::time::SystemTime;

pub mod block;
pub mod world;

/// Represents the application data file.
pub const APP_DATA_FILE: &'static str = "app_data.toml";
/// Represents the chunk data genesis file.
pub const CHUNK_DATA_GENESIS: &'static str = "chunk_genesis.toml";

lazy_static! {
  /// A globally accessible application state.
  pub static ref APPLICATION: App = App::init().unwrap();
}

/// Globally accessible, shared application data.
pub struct App
{
  /// Public-facing application data, wrapping in a mutex.
  pub data: Mutex<AppData>,
}

impl App
{
  /// Initialise the application.
  ///
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
    use crate::LogLevel;
    use std::fs::metadata;

    let mut data: Option<AppData> = None;

    if let Err(e) = metadata(&APP_DATA_FILE) {
      log!(
        LogLevel::Error,
        "App data config does not exist, it will be created."
      );

      log!(
        LogLevel::Trace,
        "Don't forget to edit the file with your desired configs!"
      );

      let _ = AppData::new().save()?;

      return Err(e.into());
    } else {
      data = Some(AppData::new().load()?);
    }

    return match data {
      None => Err(std::io::Error::from_raw_os_error(1).into()),
      Some(d) => Ok(Self {
        data: Mutex::new(d),
      }),
    };
  }

  /// Runs through the basic setup of the game.
  ///
  ///
  ///
  /// # Things to do when the player clicks "Play"
  /// 1.) Check for world seeds.
  /// 2.) Create the 'genesis' block.
  /// 3.) Generate the spawn area.
  /// 4.) Spawn the player.
  pub async fn exec_play_setup(&self) -> Result<()>
  {
    Ok(())
  }

  /// Check the run time of the game.
  ///
  ///
  /// TODO.
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
  pub time: SystemTime,
  /// The chosen name of the player.
  pub username: Option<String>,
  /// A list of worlds.
  pub worlds: Vec<String>,
}

impl AppData
{
  /// Creates a new instance of `AppData`.
  pub fn new() -> Self
  {
    AppData {
      time: SystemTime::now(),
      username: Some("USERNAME".to_string()),
      worlds: vec![],
    }
  }

  /// Saves the current instance of `AppData` to a file and
  /// returns the result of the operation.
  ///
  ///
  /// # Example
  /// ```no_run
  /// let app_data = AppData::new();
  /// if !AppData::exists() {
  ///   app_data.save();
  /// }
  /// ```
  pub fn save(&self) -> Result<()>
  {
    use std::io::Write;

    let mut fi = if std::fs::metadata(&APP_DATA_FILE).is_ok() {
      File::open(&APP_DATA_FILE)
    } else {
      File::create(&APP_DATA_FILE)
    }
    .unwrap();

    let content = toml::to_string(self).unwrap();

    fi.write_all(content.as_bytes()).unwrap();

    Ok(())
  }

  /// Loads configuration data from the app config file into the current instance of `AppData`.
  ///
  ///
  /// # Example
  /// ```no_run
  /// let mut app_data = AppData::new();
  /// if AppData::exists() {
  ///   app_data = app_data.load();
  /// }
  /// ```
  pub fn load(mut self) -> Result<Self>
  {
    use crate::LogLevel;
    use std::io::Read;

    let mut fi = if std::fs::metadata(&APP_DATA_FILE).is_ok() {
      File::open(&APP_DATA_FILE)
    } else {
      log!(LogLevel::Error, "Configuration file does not exist!");
      log!(LogLevel::Info, "Creating config...");
      &self.save().unwrap();

      Err(Error::from_raw_os_error(1))
    }
    .unwrap();

    let mut content = String::new();
    fi.read_to_string(&mut content).unwrap();
    self = toml::from_str(content.as_str()).unwrap();

    Ok(self)
  }

  /// Checks whether the application configuration file exists.
  ///
  ///
  /// # Example
  /// ```no_run
  /// let app_data = AppData::new();
  /// if !AppData::exists() {
  ///   println!("Application configuration does not exist.");
  /// }
  /// ```
  #[inline]
  pub fn exists() -> bool
  {
    use std::fs::metadata;

    if metadata(&APP_DATA_FILE).is_ok() {
      true
    } else {
      false
    }
  }
}
