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
  fn recv_forgot_password(&self, id: &str) -> Result<ValidationPair, String>;
}

impl Forgot for Account {
  fn send_forgot_password(&self, mail: &str) -> Result<(), String>
  {
    if !valid_mail(mail) {
      return Err(self.dictionary.get("general.error.invalid.mail", Language::English));
    }

    let forgot_id: String;
    let mut member_id = 0;
    {
      {
        let lower_mail = mail.to_lowercase();
        for member_entry in self.member.read().unwrap().values() {
          if member_entry.mail == lower_mail {
            member_id = member_entry.id;
            break;
          }
        }

        let member = self.member.read().unwrap();
        let entry_get = member.get(&member_id);
        if entry_get.is_none() {
          // Don't leak information about existance
          return Ok(());
        }
        let entry = entry_get.unwrap();

        forgot_id = sha3::hash(&[&member_id.to_string(), "forgot", &entry.salt]);
        if !mail::send(&entry.mail, &entry.nickname, self.dictionary.get("forgot.confirmation.subject", Language::English),
                       strformat::fmt(self.dictionary.get("forgot.confirmation.text", Language::English), &[&forgot_id])) {
          return Err(self.dictionary.get("general.error.mail_send", Language::English));
        }
      }
      if self.db_main.execute_wparams("UPDATE member SET forgot_password=1 WHERE id=:id", params!("id" => member_id)) {
        let mut member = self.member.write().unwrap();
        let entry = member.get_mut(&member_id).unwrap();
        entry.forgot_password = true;
      }
    }

    let mut forgot_password = self.forgot_password.write().unwrap();
    forgot_password.insert(forgot_id, member_id);

    Ok(())
  }

  fn recv_forgot_password(&self, id: &str) -> Result<ValidationPair, String>
  {
    let mut removable = false;
    let user_id;
    {
      let forgot_password = self.forgot_password.read().unwrap();
      match forgot_password.get(id) {
        Some(member_id) => {
          // Sending random generated password
          let user_pass = random::alphanumeric(32);
          user_id = *member_id;
          {
            let member = self.member.read().unwrap();
            let entry = member.get(member_id).unwrap();
            if !mail::send(&entry.mail, &entry.nickname, self.dictionary.get("forgot.information.subject", Language::English),
                           strformat::fmt(self.dictionary.get("forgot.information.text", Language::English), &[&user_pass])) {
              return Err(self.dictionary.get("general.error.mail_send", Language::English));
            }
          }
          if self.db_main.execute_wparams("UPDATE member SET forgot_password=0 WHERE id=:id", params!(
            "id" => *member_id
          )) {
            {
              let mut member = self.member.write().unwrap();
              let entry = member.get_mut(member_id).unwrap();
              entry.forgot_password = false;
            }
            removable = true;
            self.update_password(user_id, &user_pass);
          }
        }
        None => return Err(self.dictionary.get("forgot.error.no_forgot_issued", Language::English))
      }
    }
    if removable {
      let mut forgot_password = self.forgot_password.write().unwrap();
      forgot_password.remove(id);
      return self.create_validation(
        &self.dictionary.get("general.login", Language::English),
        user_id, time_util::get_ts_from_now_in_secs(30));
    }
    Err(self.dictionary.get("general.unknown", Language::English))
  }
}