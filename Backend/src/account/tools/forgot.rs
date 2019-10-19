use crate::Backend;
use crate::util::sha3::{hash_sha3};
use crate::util::mail::{send_mail};
use crate::util::random::{rnd_alphanumeric};

use crate::account::domainvalue::validation_pair::ValidationPair;
use crate::account::tools::account::Account;
use crate::database::tools::mysql::execute::Execute;

pub trait AccountForgot {
  fn send_forgot_password(&self, params: &ValidationPair) -> bool;
  fn recv_forgot_password(&self, id: &str) -> bool;
}

impl AccountForgot for Backend {
  fn send_forgot_password(&self, params: &ValidationPair) -> bool
  {
    if !self.validate(params) {
      return false; // Rather return errors?
    }

    let forgot_id: String;
    {
      {
        let member = self.data_acc.member.read().unwrap();
        let entry = member.get(&params.id).unwrap();
        forgot_id = hash_sha3(vec![&params.id.to_string(), "forgot", &entry.salt]);
        if !send_mail(&entry.mail, "TODO: Username", "Forgot password utility", &vec!["TODO: FANCY TEXT\nhttps://jaylapp.dev/API/account/forgot/confirm/", &forgot_id].concat()){
          return false;
        }
      }
      if !self.db_main.execute_wparams("UPDATE member SET forgot_password=1 WHERE id=:id", params!("id" => params.id)) {
        return false;
      } else {
        let mut member = self.data_acc.member.write().unwrap();
        let entry = member.get_mut(&params.id).unwrap();
        entry.forgot_password = true;
      }
    }

    let mut forgot_password = self.data_acc.forgot_password.write().unwrap();
    forgot_password.insert(forgot_id, params.id);

    true
  }

  fn recv_forgot_password(&self, id: &str) -> bool
  {
    let mut removable = false;
    {
      let forgot_password = self.data_acc.forgot_password.read().unwrap();
      match forgot_password.get(id) {
        Some(member_id) => {
          // Sending random generated password
          let new_pass = rnd_alphanumeric(16);
          {
            let member = self.data_acc.member.read().unwrap();
            let entry = member.get(member_id).unwrap();
            if send_mail(&entry.mail, "TODO: username", "TODO: Forgot password utility", &vec!["TODO: Text\n New Password: ", &new_pass].concat()) {
                return false;
            }
          }
          if self.db_main.execute_wparams("UPDATE member SET forgot_password=0, password=:pass WHERE id=:id", params!(
            "pass" => new_pass,
            "id" => *member_id
          )) {
            let mut member = self.data_acc.member.write().unwrap();
            let entry = member.get_mut(member_id).unwrap();
            entry.forgot_password = false;
            removable = true;
          }
        },
        None => return false
      }
    }
    if removable {
      let mut forgot_password = self.data_acc.forgot_password.write().unwrap();
      forgot_password.remove(id);
      return true;
    }
    false
  }
}