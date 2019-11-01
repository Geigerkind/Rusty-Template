extern crate pwned;
use pwned::api::*;
use crate::util::password::domainvalue::password_failure::PasswordFailure;

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