use crate::Backend;
use crate::util::Util;

use crate::account::domainvalue::validation_pair::ValidationPair;
use crate::account::tools::account::Account;

pub trait AccountDelete {
  fn issue_delete(&self, params: &ValidationPair) -> bool;
  fn confirm_delete(&self, id: &str) -> bool;
}

impl AccountDelete for Backend {
  fn issue_delete(&self, params: &ValidationPair) -> bool
  {
    if !self.validate(params) {
      return false; // Rather return errors?
    }

    let delete_id: String;
    {
      {
        let member = self.data_acc.member.read().unwrap();
        let entry = member.get(&params.id).unwrap();
        delete_id = Util::sha3(self, vec![&params.id.to_string(), "delete", &entry.salt]);
        if !Util::send_mail(self, &entry.mail, "TODO: Username", "Delete account utility", &vec!["TODO: FANCY TEXT\nhttps://jaylapp.dev/API/account/delete/confirm/", &delete_id].concat()){
          return false;
        }
      }
      if !self.db_main.execute_wparams("UPDATE member SET delete_account=1 WHERE id=:id", params!("id" => params.id)) {
        return false;
      } else {
        let mut member = self.data_acc.member.write().unwrap();
        let entry = member.get_mut(&params.id).unwrap();
        entry.delete_account = true;
      }
    }

    let mut delete_account = self.data_acc.delete_account.write().unwrap();
    delete_account.insert(delete_id, params.id);

    true
  }

  fn confirm_delete(&self, id: &str) -> bool
  {
    let mut removable = false;
    {
      let delete_account = self.data_acc.delete_account.read().unwrap();
      match delete_account.get(id) {
        Some(member_id) => {
          if self.db_main.execute_wparams("DELETE FROM member WHERE id = :id", params!(
            "id" => *member_id
          )) {
            let mut hash_to_member = self.data_acc.hash_to_member.write().unwrap();
            let mut member = self.data_acc.member.write().unwrap();
            self.helper_clear_validation(member_id, &mut (*hash_to_member), &mut (*member));
            member.remove(member_id);
            removable = true;
          }
        },
        None => return false
      }
    }
    if removable {
      let mut delete_account = self.data_acc.delete_account.write().unwrap();
      delete_account.remove(id);
      return true;
    }

    false
  }
}