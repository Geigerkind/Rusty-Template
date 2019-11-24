use language::domain_value::Language;
use language::tools::Get;
use mysql_connection::tools::Execute;
use str_util::{sha3, strformat};
use validator::domain_value::PasswordFailure;
use validator::tools::{valid_mail, valid_nickname, valid_password};

use crate::account::domain_value::{AccountInformation, ValidationPair};
use crate::account::material::Account;
use crate::account::tools::{GetAccountInformation, Token};

pub trait Update {
  fn change_name(&self, new_nickname: &str, member_id: u32) -> Result<AccountInformation, String>;
  fn change_password(&self, new_password: &str, member_id: u32) -> Result<ValidationPair, String>;
  fn update_password(&self, new_password: &str, member_id: u32) -> Result<(), String>;
  fn change_mail(&self, new_mail: &str, member_id: u32) -> Result<ValidationPair, String>;
}

impl Update for Account {
  fn change_name(&self, new_nickname: &str, member_id: u32) -> Result<AccountInformation, String>
  {
    if !valid_nickname(new_nickname) {
      return Err(self.dictionary.get("general.error.invalid.nickname", Language::English));
    }

    // Check if the name exists already
    let lower_name = new_nickname.to_lowercase();
    for entry in self.member.read().unwrap().values() {
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
      {
        let mut member = self.member.write().unwrap();
        let entry = member.get_mut(&member_id).unwrap();
        entry.nickname = new_nickname.to_owned();
      }
      return Ok(self.get(member_id).unwrap());
    }

    Err(self.dictionary.get("general.error.unknown", Language::English))
  }

  fn change_password(&self, new_password: &str, member_id: u32) -> Result<ValidationPair, String>
  {
    match valid_password(new_password) {
      Err(PasswordFailure::TooFewCharacters) => return Err(self.dictionary.get("general.error.password.length", Language::English)),
      Err(PasswordFailure::Pwned(num_pwned)) => return Err(strformat::fmt(self.dictionary.get("general.error.password.pwned", Language::English), &[&num_pwned.to_string()])),
      Ok(_) => ()
    };

    self.update_password(new_password, member_id)
        .and_then(|()| self.create_token(
          &self.dictionary
                      .get("general.login", Language::English), member_id, time_util::get_ts_from_now_in_secs(30))
                      .and_then(|api_token| Ok(api_token.to_validation_pair())))
  }

  fn update_password(&self, new_password: &str, member_id: u32) -> Result<(), String> {
    let hash: String;
    {
      let member = self.member.read().unwrap();
      let entry = member.get(&member_id).unwrap();
      hash = sha3::hash(&[new_password, &entry.salt]);
    }

    if self.db_main.execute_wparams("UPDATE member SET password=:password WHERE id=:id", params!(
      "password" => hash.clone(),
      "id" => member_id
    )) {
      return self.clear_tokens(member_id).and_then(|()| {
        let mut member = self.member.write().unwrap();
        let entry = member.get_mut(&member_id).unwrap();
        entry.password = hash;
        Ok(())
      });
    }
    Err(self.dictionary.get("general.error.unknown", Language::English))
  }

  fn change_mail(&self, new_mail: &str, member_id: u32) -> Result<ValidationPair, String>
  {
    if !valid_mail(new_mail) {
      return Err(self.dictionary.get("general.error.invalid.mail", Language::English));
    }

    // Check if the mail exists already
    let lower_mail = new_mail.to_lowercase();
    for entry in self.member.read().unwrap().values() {
      if entry.mail == lower_mail
        && entry.id != member_id
      {
        return Err(self.dictionary.get("update.error.mail_taken", Language::English));
      }
    }

    if self.db_main.execute_wparams("UPDATE member SET mail=:mail WHERE id=:id", params!(
      "mail" => lower_mail.clone(),
      "id" => member_id
    )) {
      return self.clear_tokens(member_id).and_then(
        |()| {
          {
            let mut member = self.member.write().unwrap();
            let entry = member.get_mut(&member_id).unwrap();
            entry.mail = lower_mail.to_owned();
          }
          self.create_token(&self.dictionary.get("general.login", Language::English),
            member_id, time_util::get_ts_from_now_in_secs(30)).and_then(|api_token| Ok(api_token.to_validation_pair()))
        });
    }
    Err(self.dictionary.get("general.error.unknown", Language::English))
  }
}