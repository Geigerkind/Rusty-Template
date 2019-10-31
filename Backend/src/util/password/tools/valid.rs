extern crate pwned;

use crate::util::strformat;
use crate::util::language::material::dictionary::Dictionary;
use crate::util::language::tools::get::Get;
use crate::util::language::domainvalue::language::Language;
use crate::util::password::language::init::Init;

use pwned::api::*;

pub fn password(input: &str) -> Result<(), String>
{
  lazy_static! {
    static ref DICTIONARY: Dictionary = {
      let dictionary = Dictionary::default();
      dictionary.init();
      dictionary
    };
    static ref PWNED: Pwned = PwnedBuilder::default().build().unwrap();
  }

  if input.len() < 12 {
    return Err(DICTIONARY.get("error.length", Language::English));
  }

  match PWNED.check_password(input) {
    Ok(pwd) => {
      if pwd.count == 0 {
        return Ok(());
      }
      return Err(strformat::fmt(DICTIONARY.get("error.pwned", Language::English), &vec![&pwd.count.to_string()]));
    },
    // Ignore this case
    Err(_) => Ok(())
  }
}