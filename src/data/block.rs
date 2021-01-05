/*!
Block-related data structures.
*/

#[derive(Clone, Deserialize, Serialize)]
/// Represents a block in the world.
pub struct Block
{
  /// ID tag of the block.
  id: u32,
  /// Related ID tags of the block.
  tags: Vec<u32>,
}

#[derive(Clone, Deserialize, Serialize)]
/// Represents a chunk in the world.
///
///
/// Filled with blocks.
pub struct Chunk
{
  /// Blocks in the chunk.
  block: Vec<Block>,
}

impl Chunk
{
  /// TODO.
  pub fn index(x: u8, y: u8, z: u8) -> u32
  {
    (x + (y * 16) + (z * 256)) as u32
  }
}

#[derive(Deserialize, Serialize)]
/// A vectorized map of the world.
pub struct Map
{
  /// Chunks displayed on the map.
  chunk: Vec<Chunk>,
  /// The x coordinate of each block.
  x: Vec<i32>,
  /// The y coordinate of each block.
  y: Vec<i32>,
  /// The z coordinate of each block.
  z: Vec<i32>,
}

impl Map
{
  /// Get the chunk at the specified coordinates.
  pub fn get_chunk(&self, x: i32, y: i32, z: i32) -> Chunk
  {
    let x: i32 = (x - (x % 16)) / 16;
    let y: i32 = (x - (x % 16)) / 16;
    let z: i32 = (x - (x % 16)) / 16;

    let mut value: Option<Chunk> = None;

    for i in 0..self.x.len() {
      if self.x[i] == x && self.y[i] == y && self.z[i] == z {
        break;
      }

      value = Some(self.chunk[i].clone());
    }

    match value {
      Some(v) => v,
      None => Self::load_chunk(x, y, z),
    }
  }

  /// Load the chunk at the specified coordinates.
  fn load_chunk(x: i32, y: i32, z: i32) -> Chunk
  {
    todo!()
  }
}
