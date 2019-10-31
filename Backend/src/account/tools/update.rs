use crate::util::sha3;
use crate::util::validator;
use crate::account::service::update::PostChangeStr;
use crate::database::tools::mysql::execute::Execute;
use crate::account::tools::validator::Validator;
use crate::account::material::account::Account;
use crate::account::domainvalue::validation_pair::ValidationPair;
use crate::account::domainvalue::account_information::AccountInformation;
use crate::account::tools::get::GetAccountInformation;
use crate::language::tools::get::Get;
use crate::language::domainvalue::language::Language;

pub trait Update {
  fn change_name(&self, params: &PostChangeStr) -> Result<AccountInformation, String>;
  fn change_password(&self, params: &PostChangeStr) -> Result<ValidationPair, String>;
  fn change_mail(&self, params: &PostChangeStr) -> Result<ValidationPair, String>;
}

impl Update for Account {
  fn change_name(&self, params: &PostChangeStr) -> Result<AccountInformation, String>
  {
    if !self.validate(&params.validation) {
      return Err(self.dictionary.get("general.error.validate", Language::English));
    }

    if !validator::nickname(&params.content) {
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

    if params.content.is_empty() {
      return Err(self.dictionary.get("general.error.invalid.password", Language::English));
    }

    let hash: String;
    {
      let member = self.member.read().unwrap();
      let entry = member.get(&params.validation.id).unwrap();
      hash = sha3::hash(vec![&entry.mail, &params.content, &entry.salt]);
    }

    if self.db_main.execute_wparams("UPDATE member SET password=:password WHERE id=:id", params!(
      "password" => hash.clone(),
      "id" => params.validation.id
    )) {
      let mut hash_to_member = self.hash_to_member.write().unwrap();
      let mut member = self.member.write().unwrap();
      self.helper_clear_validation(params.validation.id, &mut(*hash_to_member), &mut(*member));
      {
        let entry = member.get_mut(&params.validation.id).unwrap();
        entry.password = hash;
      }
      return Ok(self.helper_create_validation(params.validation.id, &mut(*hash_to_member), &mut(*member)));
    }

    Err(self.dictionary.get("general.error.unknown", Language::English))
  }

  fn change_mail(&self, params: &PostChangeStr) -> Result<ValidationPair, String>
  {
    if !self.validate(&params.validation) {
      return Err(self.dictionary.get("general.error.validate", Language::English));
    }

    if !validator::mail(&params.content) {
      return Err(self.dictionary.get("general.error.invalid.mail", Language::English));
    }

    // Check if the mail exists already
    let lower_mail = params.content.to_lowercase();
    for entry in self.member.read().unwrap().values() {
      if entry.mail.to_lowercase() == lower_mail
        && entry.id != params.validation.id
      {
        return Err(self.dictionary.get("update.error.mail_taken", Language::English));
      }
    }

    if self.db_main.execute_wparams("UPDATE member SET mail=:mail WHERE id=:id", params!(
      "mail" => params.content.clone(),
      "id" => params.validation.id
    )) {
      let mut hash_to_member = self.hash_to_member.write().unwrap();
      let mut member = self.member.write().unwrap();
      self.helper_clear_validation(params.validation.id, &mut(*hash_to_member), &mut(*member));
      {
        let entry = member.get_mut(&params.validation.id).unwrap();
        entry.mail = params.content.to_owned();
      }
      return Ok(self.helper_create_validation(params.validation.id, &mut(*hash_to_member), &mut(*member)));
    }

    Err(self.dictionary.get("general.error.unknown", Language::English))
  }
}