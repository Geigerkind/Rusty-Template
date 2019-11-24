use language::domain_value::Language;
use language::tools::Get;
use mail;
use mysql_connection::tools::Execute;
use str_util::{sha3, strformat};

use crate::account::domain_value::AccountInformation;
use crate::account::material::Account;
use crate::account::tools::{GetAccountInformation, Token};

pub trait Delete {
  fn issue_delete(&self, member_id: u32) -> Result<AccountInformation, String>;
  fn confirm_delete(&self, delete_id: &str) -> Result<(), String>;
}

impl Delete for Account {
  fn issue_delete(&self, member_id: u32) -> Result<AccountInformation, String>
  {
    if self.db_main.execute_wparams("UPDATE member SET delete_account=1 WHERE id=:id", params!("id" => member_id)) {
      let mut member = self.member.write().unwrap();
      let entry = member.get_mut(&member_id).unwrap();
      entry.delete_account = true;

      let delete_id = sha3::hash(&[&member_id.to_string(), "delete", &entry.salt]);
      let mut delete_account = self.delete_account.write().unwrap();
      delete_account.insert(delete_id.clone(), member_id);

      // Send a confirmation mail to the member now
      if !mail::send(&entry.mail, &entry.nickname, self.dictionary.get("create.confirmation.subject", Language::English),
                     strformat::fmt(self.dictionary.get("create.confirmation.text", Language::English), &[&delete_id])) {
        return Err(self.dictionary.get("general.error.mail_send", Language::English));
      }
    } else {
      return Err(self.dictionary.get("general.error.unknown", Language::English));
    }
    Ok(self.get(member_id).unwrap())
  }

  fn confirm_delete(&self, delete_id: &str) -> Result<(), String>
  {
    {
      let delete_account = self.delete_account.read().unwrap();
      match delete_account.get(delete_id) {
        Some(member_id) => {
          if self.db_main.execute_wparams("DELETE FROM member WHERE id = :id", params!(
            "id" => *member_id
          )) {
            match self.clear_tokens(*member_id) {
              Ok(_) => {
                let mut member = self.member.write().unwrap();
                member.remove(member_id);
              },
              Err(err_str) => return Err(err_str)
            }
          } else {
            return Err(self.dictionary.get("general.error.unknown", Language::English));
          }
        }
        None => return Err(self.dictionary.get("delete.error.no_delete_issued", Language::English))
      }
    }
    let mut delete_account = self.delete_account.write().unwrap();
    delete_account.remove(delete_id);
    Ok(())
  }
}