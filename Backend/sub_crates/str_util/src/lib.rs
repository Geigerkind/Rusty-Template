extern crate rand;
extern crate rand_distr;
extern crate sha3 as sha;

mod tests;
mod tools;

pub use self::tools::random;
pub use self::tools::sha3;
pub use self::tools::strformat;