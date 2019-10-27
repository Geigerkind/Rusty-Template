use crate::util::sha3;
use crate::account::service::login::PostLogin;
use crate::account::tools::validator::Validator;
use crate::account::material::account::Account;

pub trait Login {
  fn login(&self, params: &PostLogin) -> Option<String>;
}

impl Login for Account {
  fn login(&self, params: &PostLogin) -> Option<String>
  {
    // Do not change the order else we might end up in a dead lock!
    let mut hash_to_member = self.hash_to_member.write().unwrap();
    let mut member = self.member.write().unwrap();

    let lower_mail = params.mail.to_lowercase();
    let mut entry_key: u32 = 0;
    for (id, entry) in &(*member) {
      if entry.mail.to_lowercase() != lower_mail { continue; }
      if entry.password != sha3::hash(vec![&params.password, &entry.salt]) { break; } // Password is wrong
      entry_key = *id;
      break
    }
    if entry_key == 0 { return None; }

    Some(self.helper_create_validation(entry_key, &mut(*hash_to_member), &mut(*member)))
  }
}