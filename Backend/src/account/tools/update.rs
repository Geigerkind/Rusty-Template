use crate::Backend;
use crate::util::Util;

use crate::account::dto::update::PostChangeStr;
use crate::account::tools::account::Account;

pub trait AccountUpdate {
  fn change_name(&self, params: &PostChangeStr) -> bool;
  fn change_password(&self, params: &PostChangeStr) -> Option<String>;
  fn change_mail(&self, params: &PostChangeStr) -> Option<String>;
}

impl AccountUpdate for Backend {
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
    for (_, entry) in &(*self.data_acc.member.read().unwrap()) {
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
      let mut member = self.data_acc.member.write().unwrap();
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
      let member = self.data_acc.member.read().unwrap();
      let entry = member.get(&params.validation.id).unwrap();
      hash = Util::sha3(self, vec![&entry.mail, &params.content, &entry.salt]);
    }

    if self.db_main.execute_wparams("UPDATE member SET password=:password WHERE id=:id", params!(
      "password" => hash.clone(),
      "id" => params.validation.id
    )) {
      let mut hash_to_member = self.data_acc.hash_to_member.write().unwrap();
      let mut member = self.data_acc.member.write().unwrap();
      self.helper_clear_validation(&params.validation.id, &mut(*hash_to_member), &mut(*member));
      {
        let entry = member.get_mut(&params.validation.id).unwrap();
        entry.password = hash.to_owned();
      }
      return Some(self.helper_create_validation(&params.validation.id, &mut(*hash_to_member), &mut(*member)));
    }

    None
  }

  fn change_mail(&self, params: &PostChangeStr) -> Option<String>
  {
    if !self.validate(&params.validation) {
      return None; // Rather return errors?
    }

    if !Util::is_valid_mail(self, &params.content) {
      return None;
    }

    // Check if the mail exists already
    let lower_mail = params.content.to_lowercase();
    for (_, entry) in &(*self.data_acc.member.read().unwrap()) {
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
      let mut hash_to_member = self.data_acc.hash_to_member.write().unwrap();
      let mut member = self.data_acc.member.write().unwrap();
      self.helper_clear_validation(&params.validation.id, &mut(*hash_to_member), &mut(*member));
      {
        let entry = member.get_mut(&params.validation.id).unwrap();
        entry.mail = params.content.to_owned();
      }
      return Some(self.helper_create_validation(&params.validation.id, &mut(*hash_to_member), &mut(*member)));
    }

    None
  }
}