use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::ops::Add;

mod tests;

pub fn get_ts_from_now_in_secs(days: u64) -> u64 {
  SystemTime::now().add(Duration::from_secs(days*24*60*60)).duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs()
}