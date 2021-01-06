/*!
Block-related data structures.
*/


use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

lazy_static! {
  pub static ref SBHANDLER: Mutex<Vec<SBHandler<'static>>> = Mutex::new(vec![]);
  pub static ref WORLD: Option<World> = None;
}

#[repr(u32)]
pub enum BlockId
{
  Air = 0,
  Stone = 1,
  Grass = 2,
  Dirt = 3,
  Cobblestone = 4,
  WoodPlank = 5,
  Sapling = 6,
  Bedrock = 7,
  Water = 8,
  Lava = 9,
}

#[derive(Clone, Deserialize, Serialize)]
/// Represents a block in the world.
pub struct Block
{
  id: BlockId,
  variant: u32,
  tag: Vec<u32>,
}

pub trait SmartBlock
{
  fn update(self);
}

pub struct SBHandler<'a>
{
  chunk: &'a mut Chunk,
  smart_block: Box<Vec<dyn SmartBlock>>,
  block_index: Vec<usize>,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Liquid
{
  source_distance: u32
}

impl SmartBlock for Liquid
{
  fn update(self) {}
}

#[derive(Clone, Deserialize, Serialize)]
/// Represents a chunk in the world.
///
///
/// Filled with blocks.
pub struct Chunk
{
  x: i32,
  y: i32,
  z: i32,
  block: Vec<Block>,
  sb_handler_index: usize,
}

impl Chunk
{
  pub fn index(x: u32, y: u32, z: u32) -> usize
  {
    (x + (y * 16) + (z * 256)) as usize
  }
  
  pub fn adj(&self, index: u32) -> Vec<Block>
  {
    let index = index as f32;
    let mut val: Vec<Block> = vec![];
    if (index % 16) + 1 > 15
    {
      val.push(WORLD.unwrap().chunk_overflow(&self, index - 15));
    } else
    {
      val.push(self.block[index + 1]);
    }
    if (index % 16) - 1 < 0
    {
      val.push(WORLD.unwrap().chunk_overflow(&self, index + 15));
    } else
    {
      val.push(self.block[index - 1]);
    }

    if ((index % 256) / 16) as f32.floor() + 1 > 15
    {
      val.push(WORLD.unwrap().chunk_overflow(&self, index - 240));
    } else
    {
      val.push(self.block[index + 16]);
    }
    if ((index % 256) / 16) as f32.floor() - 1 < 0
    {
      val.push(WORLD.unwrap().chunk_overflow(&self, index + 240));
    } else
    {
      val.push(self.block[index - 16]);
    }

    if (index / 256) as f32.floor() + 1 > 15
    {
      val.push(WORLD.unwrap().chunk_overflow(&self, index - 240));
    } else
    {
      val.push(self.block[index + 16]);
    }
    if (index / 256) as f32.floor() - 1 < 0
    {
      val.push(WORLD.unwrap().chunk_overflow(&self, index + 240));
    } else
    {
      val.push(self.block[index - 16]);
    }

    val

  }
}

#[derive(Deserialize, Serialize)]
/// A vectorized map of the world.
pub struct World
{
  /// Chunks displayed in the world.
  chunk: Vec<Chunk>,
}

impl World
{
  /// Get the index of the chunk at the specified coordinates.
  pub fn get_chunk(&self, x: i32, y: i32, z: i32) -> usize
  {
    let mut value: Option<usize> = None;
    
    for i in 0..self.x.len()
    {
      if self.x[i] == x
          && self.y[i] == y
          && self.z[i] == z
      {
        value = Some(i);
        break;
      }
    }
    
    match value {
      Some(v) => v,
      None => Self::load_chunk(&mut WORLD.unwrap(), x, y, z)
    }
  }

  pub fn load_chunk(&mut self, x: i32, y: i32, z: i32) -> usize
  {
    //This needs to be replaced with proper chunk building.
    let mut chunk = Chunk
    {
      x,
      y,
      z,
      block: vec![],
      sb_handler_index: *SBHANDLER.len(),
    };

    let mut handler: SBHandler = SBHandler
    {
      chunk: &mut chunk,
      smart_block: Box::from(vec![]),
      block_index: vec![],
    };

    let mut i = 0;

    for block in chunk.block
    {
      match block.id {
        BlockId::Water | BlockId::Lava =>
          {
            handler.smart_block.push(Liquid { source_distance: block.variant });
            handler.block_index.push(i);
          }
        _ => continue
      }

      i += 1;
    }

    *SBHANDLER.push(handler);
    self.chunk.push(chunk);


    self.chunk.len() - 1
  }

  pub fn unload_chunk(x: i32, y: i32, z: i32)
  {
    todo!()
  }

  pub fn chunk_overflow(chunk : &Chunk, block : usize){
    todo!()
  }
}
