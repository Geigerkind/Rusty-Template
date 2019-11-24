use language::domain_value::Language;
use language::tools::Get;
use mysql_connection::tools::Execute;
use str_util::{sha3, strformat};
use validator::tools::{valid_mail, valid_nickname, valid_password};
use validator::domain_value::PasswordFailure;

use crate::account::domain_value::{AccountInformation, ValidationPair, UpdateContent, UpdateContentCredentials};
use crate::account::material::Account;
use crate::account::tools::{GetAccountInformation, Login, Token};

pub trait Update {
  fn change_name(&self, params: &UpdateContent) -> Result<AccountInformation, String>;
  fn change_password(&self, params: &UpdateContent) -> Result<ValidationPair, String>;
  fn update_password(&self, member_id: u32, new_password: &str) -> bool;
  fn change_mail(&self, params: &UpdateContentCredentials) -> Result<ValidationPair, String>;
}

impl Update for Account {
  fn change_name(&self, params: &UpdateContent) -> Result<AccountInformation, String>
  {
    if !self.validate(&params.validation) {
      return Err(self.dictionary.get("general.error.validate", Language::English));
    }

    if !valid_nickname(&params.content) {
      return Err(self.dictionary.get("general.error.invalid.nickname", Language::English));
    }

    // Check if the name exists already
    let lower_name = params.content.to_lowercase();
    for entry in self.member.read().unwrap().values() {
      if entry.nickname.to_lowercase() == lower_name
        && entry.id != params.validation.member_id
      {
        return Err(self.dictionary.get("update.error.name_taken", Language::English));
      }
    }

    if self.db_main.execute_wparams("UPDATE member SET nickname=:nickname WHERE id=:id", params!(
      "nickname" => params.content.clone(),
      "id" => params.validation.member_id
    )) {
      {
        let mut member = self.member.write().unwrap();
        let entry = member.get_mut(&params.validation.member_id).unwrap();
        entry.nickname = params.content.to_owned();
      }
      return Ok(self.get(params.validation.member_id).unwrap());
    }

    Err(self.dictionary.get("general.error.unknown", Language::English))
  }

  fn change_password(&self, params: &UpdateContent) -> Result<ValidationPair, String>
  {
    if !self.validate(&params.validation) {
      return Err(self.dictionary.get("general.error.validate", Language::English));
    }

    match valid_password(&params.content) {
      Err(PasswordFailure::TooFewCharacters) => return Err(self.dictionary.get("general.error.password.length", Language::English)),
      Err(PasswordFailure::Pwned(num_pwned)) => return Err(strformat::fmt(self.dictionary.get("general.error.password.pwned", Language::English), &[&num_pwned.to_string()])),
      Ok(_) => ()
    };

    if self.update_password(params.validation.member_id, &params.content) {
      return self.create_validation_unsafe(&self.dictionary.get("general.login", Language::English), params.validation.member_id, time_util::get_ts_from_now_in_secs(30));
    }

    Err(self.dictionary.get("general.error.unknown", Language::English))
  }

  fn update_password(&self, member_id: u32, new_password: &str) -> bool
  {
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
      match self.clear_tokens(member_id) {
        Ok(_) => {
          let mut member = self.member.write().unwrap();
          let entry = member.get_mut(&member_id).unwrap();
          entry.password = hash;
        }
        Err(_) => return false
      }
      return true;
    }
    false
  }

  fn change_mail(&self, params: &UpdateContentCredentials) -> Result<ValidationPair, String>
  {
    let validation = self.login(&params.credentials);
    if validation.is_err() {
      return Err(self.dictionary.get("general.error.validate", Language::English));
    }
    let validation_pair = validation.unwrap();

    if !valid_mail(&params.content) {
      return Err(self.dictionary.get("general.error.invalid.mail", Language::English));
    }

    // Check if the mail exists already
    let lower_mail = params.content.to_lowercase();
    for entry in self.member.read().unwrap().values() {
      if entry.mail == lower_mail
        && entry.id != validation_pair.member_id
      {
        return Err(self.dictionary.get("update.error.mail_taken", Language::English));
      }
    }

    if self.db_main.execute_wparams("UPDATE member SET mail=:mail WHERE id=:id", params!(
      "mail" => lower_mail.clone(),
      "id" => validation_pair.member_id
    )) {
      match self.clear_tokens(validation_pair.member_id) {
        Ok(_) => {
          let mut member = self.member.write().unwrap();
          let entry = member.get_mut(&validation_pair.member_id).unwrap();
          entry.mail = lower_mail.to_owned();
        }
        Err(err_str) => return Err(err_str)
      }
      return self.create_validation_unsafe(
        &self.dictionary.get("general.login", Language::English),
        validation_pair.member_id, time_util::get_ts_from_now_in_secs(30));
    }

    Err(self.dictionary.get("general.error.unknown", Language::English))
  }
}