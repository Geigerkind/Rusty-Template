extern crate regex;

use regex::Regex;

pub fn mail(input: &str) -> bool
{
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^([\w+\.]+)@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.)|(([\w-]+\.)+))([a-zA-Z]{2,4}|[0-9]{1,3})(\]?)$").unwrap();
  }
  RE.is_match(input)
}

pub fn nickname(input: &str) -> bool
{
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^([a-zA-Z0-9]+)$").unwrap();
  }
  RE.is_match(input)
}