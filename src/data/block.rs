#[derive(Clone, Deserialize, Serialize)]
pub struct Block
{
  id: u32,
  tags: Vec<u32>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Chunk
{
  block: Vec<Block>
}

impl Chunk
{
  pub fn index(x: u8, y: u8, z: u8) -> u32
  {
    (x + (y * 16) + (z * 256)) as u32
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
  pub fn get_chunk(&self, x: i32, y: i32, z: i32) -> Chunk
  {
    let x: i32 = (x - (x % 16)) / 16;
    let y: i32 = (x - (x % 16)) / 16;
    let z: i32 = (x - (x % 16)) / 16;

    static mut VALUE: Option<Chunk> = None;

    for i in 0..self.x.len()
    {
      if     self.x[i] == x
          && self.y[i] == y
          && self.z[i] == z
      {
        break;
      }

      unsafe {
        VALUE = Some(self.chunk[i].clone());
      }
    }

    unsafe {
      match VALUE.clone() {
        Some(v) => v,
        None => Self::load_chunk(x, y, z)
      }
    }
  }

  fn load_chunk(x: i32, y: i32, z: i32) -> Chunk
  {
    todo!()
  }
}
