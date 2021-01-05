/*!
Game-world related data structures.
*/

#[derive(Debug, Deserialize, Serialize)]
/// Represents the game-world and its characteristics.
pub struct World
{
  /// Width of the world, in blocks.
  width: u128,
  /// Length of the world, in blocks.
  length: u128,
  /// Height of the world, in blocks.
  height: u128,
  /// Name of the world.
  name: String,
  /// Optional world-seed to determine certain characteristics of the world.
  seed: Option<String>,
}
