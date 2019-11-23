use language::domainvalue::language::Language;
use language::get::Get;
use mail;
use str_util::{sha3, strformat};

use crate::account::domainvalue::account_information::AccountInformation;
use crate::account::domainvalue::validation_pair::ValidationPair;
use crate::account::material::account::Account;
use crate::account::tools::get::GetAccountInformation;
use crate::account::tools::token::Token;
use crate::database::tools::mysql::execute::Execute;

pub trait Delete {
  fn issue_delete(&self, params: &ValidationPair) -> Result<AccountInformation, String>;
  fn confirm_delete(&self, id: &str) -> Result<(), String>;
}

impl Delete for Account {
  fn issue_delete(&self, params: &ValidationPair) -> Result<AccountInformation, String>
  {
    if !self.validate(params) {
      return Err(self.dictionary.get("general.error.validate", Language::English));
    }

    let delete_id: String;
    {
      {
        let member = self.member.read().unwrap();
        let entry = member.get(&params.member_id).unwrap();
        delete_id = sha3::hash(&[&params.member_id.to_string(), "delete", &entry.salt]);
        if !mail::send(&entry.mail, &entry.nickname, self.dictionary.get("create.confirmation.subject", Language::English),
                       strformat::fmt(self.dictionary.get("create.confirmation.text", Language::English), &[&delete_id])) {
          return Err(self.dictionary.get("general.error.mail_send", Language::English));
        }
      }
      if self.db_main.execute_wparams("UPDATE member SET delete_account=1 WHERE id=:id", params!("id" => params.member_id)) {
        let mut member = self.member.write().unwrap();
        let entry = member.get_mut(&params.member_id).unwrap();
        entry.delete_account = true;
      }
    }

    {
      let mut delete_account = self.delete_account.write().unwrap();
      delete_account.insert(delete_id, params.member_id);
    }

    Ok(self.get(params.member_id).unwrap())
  }

  fn confirm_delete(&self, id: &str) -> Result<(), String>
  {
    let mut removable = false;
    {
      let delete_account = self.delete_account.read().unwrap();
      match delete_account.get(id) {
        Some(member_id) => {
          if self.db_main.execute_wparams("DELETE FROM member WHERE id = :id", params!(
            "id" => *member_id
          )) {
            match self.clear_tokens(*member_id) {
              Ok(_) => {
                let mut member = self.member.write().unwrap();
                member.remove(member_id);
                removable = true;
              }
              Err(err_str) => return Err(err_str)
            }
          }
        }
        None => return Err(self.dictionary.get("delete.error.no_delete_issued", Language::English))
      }
    }
    if removable {
      let mut delete_account = self.delete_account.write().unwrap();
      delete_account.remove(id);
      return Ok(());
    }

    Err(self.dictionary.get("delete.error.user_not_removable", Language::English))
  }
}