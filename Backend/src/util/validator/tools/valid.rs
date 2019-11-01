extern crate pwned;
extern crate regex;

use regex::Regex;
use pwned::api::*;
use crate::util::validator::domainvalue::password_failure::PasswordFailure;

pub fn password(input: &str) -> Result<(), PasswordFailure>
{
  lazy_static! {
    static ref PWNED: Pwned = PwnedBuilder::default().build().unwrap();
  }

  if input.len() < 12 {
    return Err(PasswordFailure::TooFewCharacters);
  }

  match PWNED.check_password(input) {
    Ok(pwd) => {
      if pwd.count == 0 {
        return Ok(());
      }
      return Err(PasswordFailure::Pwned(pwd.count));
    },
    // Ignore this case
    Err(_) => Ok(())
  }
}

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