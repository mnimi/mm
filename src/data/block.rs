use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

lazy_static! {
  pub static ref SBHANDLER: Mutex<Vec<SBHandler<'static>>> = Mutex::new(vec![]);
  pub static ref MAP: Option<Map> = None;
}

#[repr(u32)]
pub enum Id
{
  Air,
  Stone,
  Grass,
  Dirt,
  Cobble,
  WoodPlank,
  Sapling,
  Bedrock,
  Water,
  Lava,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Block
{
  id: block_id,
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
  pub fn index(x: u32, y: u32, z: u32) -> u32
  {
    (x + (y * 16) + (z * 256))
  }
  pub fn adj(&self, index: u32) {
    let mut val: Vec<Block> = vec![];
    let mut remainder = index;
  }
}

#[derive(Deserialize, Serialize)]
pub struct Map
{
  chunk: Vec<Chunk>,
  x: Vec<i32>,
  y: Vec<i32>,
  z: Vec<i32>,
}

impl Map
{
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
      None => Self::load_chunk(&mut MAP.unwrap(), x, y, z)
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
        Id::Water | Id::Lava =>
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

  fn unload_chunk(x: i32, y: i32, z: i32)
  {
    todo!()
  }
}
