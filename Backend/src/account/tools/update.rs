use crate::util::str_util::tools::{sha3, strformat};
use crate::util::validator::tools::valid;
use crate::util::validator::domainvalue::password_failure::PasswordFailure;
use crate::account::service::update::PostChangeStr;
use crate::database::tools::mysql::execute::Execute;
use crate::account::tools::validator::Validator;
use crate::account::material::account::Account;
use crate::account::domainvalue::validation_pair::ValidationPair;
use crate::account::domainvalue::account_information::AccountInformation;
use crate::account::tools::get::GetAccountInformation;
use crate::util::language::tools::get::Get;
use crate::util::language::domainvalue::language::Language;

pub trait Update {
  fn change_name(&self, params: &PostChangeStr) -> Result<AccountInformation, String>;
  fn change_password(&self, params: &PostChangeStr) -> Result<ValidationPair, String>;
  fn update_password(&self, member_id: u32, new_password: &str) -> bool;
  fn change_mail(&self, params: &PostChangeStr) -> Result<ValidationPair, String>;
}

impl Update for Account {
  fn change_name(&self, params: &PostChangeStr) -> Result<AccountInformation, String>
  {
    if !self.validate(&params.validation) {
      return Err(self.dictionary.get("general.error.validate", Language::English));
    }

    if !valid::nickname(&params.content) {
      return Err(self.dictionary.get("general.error.invalid.nickname", Language::English));
    }

    // Check if the name exists already
    let lower_name = params.content.to_lowercase();
    for entry in self.member.read().unwrap().values() {
      if entry.nickname.to_lowercase() == lower_name
        && entry.id != params.validation.id
      {
        return Err(self.dictionary.get("update.error.name_taken", Language::English));
      }
    }

    if self.db_main.execute_wparams("UPDATE member SET nickname=:nickname WHERE id=:id", params!(
      "nickname" => params.content.clone(),
      "id" => params.validation.id
    )) {
      {
        let mut member = self.member.write().unwrap();
        let entry = member.get_mut(&params.validation.id).unwrap();
        entry.nickname = params.content.to_owned();
      }
      return Ok(self.get(params.validation.id).unwrap());
    }

    Err(self.dictionary.get("general.error.unknown", Language::English))
  }

  fn change_password(&self, params: &PostChangeStr) -> Result<ValidationPair, String>
  {
    if !self.validate(&params.validation) {
      return Err(self.dictionary.get("general.error.validate", Language::English));
    }

    match valid::password(&params.content) {
      Err(PasswordFailure::TooFewCharacters) => return Err(self.dictionary.get("general.error.password.length", Language::English)),
      Err(PasswordFailure::Pwned(num_pwned)) => return Err(strformat::fmt(self.dictionary.get("general.error.password.pwned", Language::English), &[&num_pwned.to_string()])),
      Ok(_) => ()
    };

    if self.update_password(params.validation.id, &params.content) {
      return Ok(self.helper_create_validation(params.validation.id));
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
      self.helper_clear_validation(member_id);
      {
        let mut member = self.member.write().unwrap();
        let entry = member.get_mut(&member_id).unwrap();
        entry.password = hash;
      }
      return true;
    }
    false
  }

  fn change_mail(&self, params: &PostChangeStr) -> Result<ValidationPair, String>
  {
    if !self.validate(&params.validation) {
      return Err(self.dictionary.get("general.error.validate", Language::English));
    }

    if !valid::mail(&params.content) {
      return Err(self.dictionary.get("general.error.invalid.mail", Language::English));
    }

    // Check if the mail exists already
    let lower_mail = params.content.to_lowercase();
    for entry in self.member.read().unwrap().values() {
      if entry.mail == lower_mail
        && entry.id != params.validation.id
      {
        return Err(self.dictionary.get("update.error.mail_taken", Language::English));
      }
    }

    if self.db_main.execute_wparams("UPDATE member SET mail=:mail WHERE id=:id", params!(
      "mail" => lower_mail.clone(),
      "id" => params.validation.id
    )) {
      self.helper_clear_validation(params.validation.id);
      {
        let mut member = self.member.write().unwrap();
        let entry = member.get_mut(&params.validation.id).unwrap();
        entry.mail = lower_mail.to_owned();
      }
      return Ok(self.helper_create_validation(params.validation.id));
    }

    Err(self.dictionary.get("general.error.unknown", Language::English))
  }
}