use language::domain_value::Language;
use language::tools::Get;
use mysql_connection::tools::Execute;
use str_util::{sha3, strformat};
use validator::domain_value::PasswordFailure;
use validator::tools::{valid_mail, valid_nickname, valid_password};

use crate::account::domain_value::AccountInformation;
use crate::account::material::{Account, APIToken};
use crate::account::tools::{GetAccountInformation, Token};

pub trait Update {
  fn change_name(&self, new_nickname: &str, member_id: u32) -> Result<AccountInformation, String>;
  fn change_password(&self, new_password: &str, member_id: u32) -> Result<APIToken, String>;
  fn update_password(&self, new_password: &str, member_id: u32) -> Result<(), String>;
  fn request_change_mail(&self, new_mail: &str, member_id: u32) -> Result<(), String>;
  fn confirm_change_mail(&self, confirmation_id: &str) -> Result<APIToken, String>;
}

impl Update for Account {
  fn change_name(&self, new_nickname: &str, member_id: u32) -> Result<AccountInformation, String>
  {
    if !valid_nickname(new_nickname) {
      return Err(self.dictionary.get("general.error.invalid.nickname", Language::English));
    }

    {
      let mut member = self.member.write().unwrap();
      // Check if the name exists already
      let lower_name = new_nickname.to_lowercase();
      for entry in member.values() {
        if entry.nickname.to_lowercase() == lower_name
          && entry.id != member_id
        {
          return Err(self.dictionary.get("update.error.name_taken", Language::English));
        }
      }

      if self.db_main.execute_wparams("UPDATE member SET nickname=:nickname WHERE id=:id", params!(
      "nickname" => new_nickname.clone(),
      "id" => member_id
    )) {
        let entry = member.get_mut(&member_id).unwrap();
        entry.nickname = new_nickname.to_owned();
      } else {
        return Err(self.dictionary.get("general.error.unknown", Language::English));
      }
    }

    Ok(self.get(member_id).unwrap())
  }

  fn change_password(&self, new_password: &str, member_id: u32) -> Result<APIToken, String>
  {
    match valid_password(new_password) {
      Err(PasswordFailure::TooFewCharacters) => return Err(self.dictionary.get("general.error.password.length", Language::English)),
      Err(PasswordFailure::Pwned(num_pwned)) => return Err(strformat::fmt(self.dictionary.get("general.error.password.pwned", Language::English), &[&num_pwned.to_string()])),
      Ok(_) => ()
    };

    self.update_password(new_password, member_id)
        .and_then(|()| self.create_token(
          &self.dictionary
                      .get("general.login", Language::English), member_id, time_util::get_ts_from_now_in_secs(30)))
  }

  fn update_password(&self, new_password: &str, member_id: u32) -> Result<(), String> {
    let mut member = self.member.write().unwrap();

    let hash: String;
    {
      let entry = member.get(&member_id).unwrap();
      hash = sha3::hash(&[new_password, &entry.salt]);
    }

    if self.db_main.execute_wparams("UPDATE member SET password=:password WHERE id=:id", params!(
      "password" => hash.clone(),
      "id" => member_id
    )) {
      return self.clear_tokens(member_id).and_then(|()| {
        let entry = member.get_mut(&member_id).unwrap();
        entry.password = hash;
        Ok(())
      });
    }
    Err(self.dictionary.get("general.error.unknown", Language::English))
  }

  fn request_change_mail(&self, new_mail: &str, member_id: u32) -> Result<(), String>
  {
    if !valid_mail(new_mail) {
      return Err(self.dictionary.get("general.error.invalid.mail", Language::English));
    }

    let mut requires_mail_confirmation = self.requires_mail_confirmation.write().unwrap();
    let mut member = self.member.write().unwrap();

    // Check if the mail exists already
    let lower_mail = new_mail.to_lowercase();
    for entry in member.values() {
      if (entry.mail == lower_mail || entry.new_mail == lower_mail)
        && entry.id != member_id
      {
        return Err(self.dictionary.get("update.error.mail_taken", Language::English));
      }
    }

    let member_entry = member.get_mut(&member_id).unwrap();
    member_entry.new_mail = lower_mail.to_owned();
    requires_mail_confirmation.insert(sha3::hash(&[&member_id.to_string(), "new_mail", &member_entry.salt]), member_id);

    Ok(())
  }

  fn confirm_change_mail(&self, confirmation_id: &str) -> Result<APIToken, String> {
    let requires_mail_confirmation = self.requires_mail_confirmation.read().unwrap();
    match requires_mail_confirmation.get(confirmation_id) {
      Some(member_id) => {
        {
          let mut member = self.member.write().unwrap();
          let member_entry = member.get_mut(member_id).unwrap();
          let lower_mail = member_entry.new_mail.clone();
          if self.clear_tokens(*member_id).is_ok() && self.db_main.execute_wparams("UPDATE member SET mail=:mail WHERE id=:id", params!(
            "mail" => lower_mail.clone(),
            "id" => member_id
          )) {
            member_entry.mail = lower_mail.to_owned();
            member_entry.new_mail = String::new();
          } else {
            return Err(self.dictionary.get("general.error.unknown", Language::English));
          }
        }
        self.create_token(&self.dictionary.get("general.login", Language::English),
                          *member_id, time_util::get_ts_from_now_in_secs(30))
      },
      None => Err(self.dictionary.get("general.error.unknown", Language::English))
    }
  }
}