use language::domain_value::Language;
use language::tools::Get;
use mail;
use mysql_connection::tools::Execute;
use str_util::{random, sha3, strformat};
use validator::tools::valid_mail;

use crate::account::domain_value::ValidationPair;
use crate::account::material::Account;
use crate::account::tools::{Token, Update};

pub trait Forgot {
  fn send_forgot_password(&self, mail: &str) -> Result<(), String>;
  fn recv_forgot_password(&self, forgot_id: &str) -> Result<ValidationPair, String>;
}

impl Forgot for Account {
  fn send_forgot_password(&self, mail: &str) -> Result<(), String>
  {
    if !valid_mail(mail) {
      return Err(self.dictionary.get("general.error.invalid.mail", Language::English));
    }

    let mut member_id = None;
    {
      let lower_mail = mail.to_lowercase();
      for member_entry in self.member.read().unwrap().values() {
        if member_entry.mail == lower_mail {
          member_id = Some(member_entry.id);
          break;
        }
      }
    }

    if member_id.is_none() {
      return Ok(()) // Don't leak information about existence
    }

    let unwrapped_member_id = member_id.unwrap();
    if self.db_main.execute_wparams("UPDATE member SET forgot_password=1 WHERE id=:id", params!("id" => unwrapped_member_id)) {

      let mut member = self.member.write().unwrap();
      let mut forgot_password = self.forgot_password.write().unwrap();
      let entry = member.get_mut(&unwrapped_member_id).unwrap();
      let forgot_id = sha3::hash(&[&unwrapped_member_id.to_string(), "forgot", &entry.salt]);

      entry.forgot_password = true;
      forgot_password.insert(forgot_id.clone(), unwrapped_member_id);

      // Only send a mail if we really set up the internal structures properly
      if !mail::send(&entry.mail, &entry.nickname, self.dictionary.get("forgot.confirmation.subject", Language::English),
                     strformat::fmt(self.dictionary.get("forgot.confirmation.text", Language::English), &[&forgot_id])) {
        return Err(self.dictionary.get("general.error.unknown", Language::English));
      }

      return Ok(())
    }
    Err(self.dictionary.get("general.error.unknown", Language::English))
  }

  fn recv_forgot_password(&self, forgot_id: &str) -> Result<ValidationPair, String>
  {
    let user_id;
    {
      let forgot_password = self.forgot_password.read().unwrap();
      match forgot_password.get(forgot_id) {
        Some(member_id) => {
          user_id = *member_id;
          if self.db_main.execute_wparams("UPDATE member SET forgot_password=0 WHERE id=:id", params!(
            "id" => *member_id
          )) {
            let mut member = self.member.write().unwrap();
            let entry = member.get_mut(member_id).unwrap();
            entry.forgot_password = false;
          } else {
            return Err(self.dictionary.get("general.error.unknown", Language::English));
          }
        }
        None => return Err(self.dictionary.get("forgot.error.no_forgot_issued", Language::English))
      }
    }

    let user_pass = random::alphanumeric(32);
    return self.update_password(&user_pass, user_id)
      .and_then(|()| {
        // Scoping because of the create_token function
        {
          let member = self.member.read().unwrap();
          let entry = member.get(&user_id).unwrap();
          if !mail::send(&entry.mail, &entry.nickname, self.dictionary.get("forgot.information.subject", Language::English),
                         strformat::fmt(self.dictionary.get("forgot.information.text", Language::English), &[&user_pass])) {
            return Err(self.dictionary.get("general.error.mail_send", Language::English));
          }
        }

        let mut forgot_password = self.forgot_password.write().unwrap();
        forgot_password.remove(forgot_id);
        return self.create_token(
          &self.dictionary.get("general.login", Language::English),
          user_id, time_util::get_ts_from_now_in_secs(30)).and_then(|api_token| Ok(api_token.to_validation_pair()));
      });
  }
}