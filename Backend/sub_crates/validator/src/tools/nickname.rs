use regex::Regex;

pub fn valid_nickname(input: &str) -> bool
{
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^([a-zA-Z0-9]+)$").unwrap();
  }
  RE.is_match(input)
}