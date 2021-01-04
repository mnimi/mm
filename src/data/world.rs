#[derive(Debug, Deserialize, Serialize)]
pub struct World
{
  width: u128,
  length: u128,
  height: u128,
  name: String,
  seed: u64,
}

