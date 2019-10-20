use crate::util::sha3;
use crate::util::validator;
use crate::account::dto::update::PostChangeStr;
use crate::database::tools::mysql::execute::Execute;
use crate::account::tools::validator::Validator;
use crate::account::material::account::Account;

pub trait Update {
  fn change_name(&self, params: &PostChangeStr) -> bool;
  fn change_password(&self, params: &PostChangeStr) -> Option<String>;
  fn change_mail(&self, params: &PostChangeStr) -> Option<String>;
}

impl Update for Account {
  fn change_name(&self, params: &PostChangeStr) -> bool
  {
    if !self.validate(&params.validation) {
      return false; // Rather return errors?
    }

    if params.content.is_empty() {
      return false;
    }

    // Check if the name exists already
    let lower_name = params.content.to_lowercase();
    for entry in self.member.read().unwrap().values() {
      if entry.nickname.to_lowercase() == lower_name
        && entry.id != params.validation.id
      {
        return false;
      }
    }

    if self.db_main.execute_wparams("UPDATE member SET nickname=:nickname WHERE id=:id", params!(
      "nickname" => params.content.clone(),
      "id" => params.validation.id
    )) {
      let mut member = self.member.write().unwrap();
      let entry = member.get_mut(&params.validation.id).unwrap();
      entry.nickname = params.content.to_owned();
      return true;
    }

    false
  }

  fn change_password(&self, params: &PostChangeStr) -> Option<String>
  {
    if !self.validate(&params.validation) {
      return None; // Rather return errors?
    }

    if params.content.is_empty() {
      return None;
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
      return Some(self.helper_create_validation(params.validation.id, &mut(*hash_to_member), &mut(*member)));
    }

    None
  }

  fn change_mail(&self, params: &PostChangeStr) -> Option<String>
  {
    if !self.validate(&params.validation) {
      return None; // Rather return errors?
    }

    if !validator::mail(&params.content) {
      return None;
    }

    // Check if the mail exists already
    let lower_mail = params.content.to_lowercase();
    for entry in self.member.read().unwrap().values() {
      if entry.mail.to_lowercase() == lower_mail
        && entry.id != params.validation.id
      {
        return None;
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
      return Some(self.helper_create_validation(params.validation.id, &mut(*hash_to_member), &mut(*member)));
    }

    None
  }
}