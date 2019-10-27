use sha3::{Digest, Sha3_512};

pub fn hash(input: Vec<&str>) -> String
{
  let mut hasher = Sha3_512::new();
  hasher.input(input.concat());
  format!("{:x}", hasher.result())
}