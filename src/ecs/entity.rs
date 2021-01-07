//! Entity type
use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::io::Write;

pub trait Entity
{
  fn to_string(&self) -> String;
}

pub struct EntityId<T>
where
    T: Entity + Sized,
{
  hash: dyn Hash
}

impl<T : Entity> EntityId<T>
{
  pub fn new(entity: T) -> Box<Self>
  where
      T: Sized
  {
    let mut hasher = DefaultHasher::new();
    hasher.write(entity.to_string().as_bytes());


    Box::new(Self { hash: hasher.finish() })
  }
}
