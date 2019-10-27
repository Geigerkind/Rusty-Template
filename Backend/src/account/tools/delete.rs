use crate::util::sha3;
use crate::util::mail;
use crate::util::strformat;
use crate::account::domainvalue::validation_pair::ValidationPair;
use crate::account::tools::validator::Validator;
use crate::account::material::account::Account;
use crate::database::tools::mysql::execute::Execute;
use crate::language::tools::get::Get;
use crate::language::domainvalue::language::Language;

pub trait Delete {
  fn issue_delete(&self, params: &ValidationPair) -> bool;
  fn confirm_delete(&self, id: &str) -> bool;
}

impl Delete for Account {
  fn issue_delete(&self, params: &ValidationPair) -> bool
  {
    if !self.validate(params) {
      return false; // Rather return errors?
    }

    let delete_id: String;
    {
      {
        let member = self.member.read().unwrap();
        let entry = member.get(&params.id).unwrap();
        delete_id = sha3::hash(vec![&params.id.to_string(), "delete", &entry.salt]);
        if !mail::send(&entry.mail, "TODO: Username", self.dictionary.get("create.confirmation.subject", Language::English),
          strformat::fmt(self.dictionary.get("create.confirmation.text", Language::English), &vec![&delete_id])){
            return false;
        }
      }
      if !self.db_main.execute_wparams("UPDATE member SET delete_account=1 WHERE id=:id", params!("id" => params.id)) {
        return false;
      } else {
        let mut member = self.member.write().unwrap();
        let entry = member.get_mut(&params.id).unwrap();
        entry.delete_account = true;
      }
    }

    let mut delete_account = self.delete_account.write().unwrap();
    delete_account.insert(delete_id, params.id);

    true
  }

  fn confirm_delete(&self, id: &str) -> bool
  {
    let mut removable = false;
    {
      let delete_account = self.delete_account.read().unwrap();
      match delete_account.get(id) {
        Some(member_id) => {
          if self.db_main.execute_wparams("DELETE FROM member WHERE id = :id", params!(
            "id" => *member_id
          )) {
            let mut hash_to_member = self.hash_to_member.write().unwrap();
            let mut member = self.member.write().unwrap();
            self.helper_clear_validation(*member_id, &mut (*hash_to_member), &mut (*member));
            member.remove(member_id);
            removable = true;
          }
        },
        None => return false
      }
    }
    if removable {
      let mut delete_account = self.delete_account.write().unwrap();
      delete_account.remove(id);
      return true;
    }

    false
  }
}