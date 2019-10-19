use crate::Backend;
use crate::util::sha3::{hash_sha3};
use crate::util::random::{rnd_alphanumeric};

use crate::account::domainvalue::account_information::AccountInformation;
use crate::account::domainvalue::validation_pair::ValidationPair;
use crate::account::material::member::Member;
use crate::database::tools::mysql::select::Select;
use crate::database::tools::mysql::execute::Execute;

use std::collections::HashMap;

pub trait Account {
  fn init(&self);
  fn validate(&self, params: &ValidationPair) -> bool;

  fn get(&self, id: u32) -> Option<AccountInformation>;

  fn helper_clear_validation(&self, member_id: &u32, hash_to_member: &mut HashMap<String, u32>, member: &mut HashMap<u32, Member>);
  fn helper_create_validation(&self, member_id: &u32, hash_to_member: &mut HashMap<String, u32>, member: &mut HashMap<u32, Member>) -> String;
}

impl Account for Backend {
  fn init(&self)
  {
    let mut requires_mail_confirmation = self.data_acc.requires_mail_confirmation.write().unwrap();
    let mut forgot_password = self.data_acc.forgot_password.write().unwrap();
    let mut delete_account = self.data_acc.delete_account.write().unwrap();
    let mut hash_to_member = self.data_acc.hash_to_member.write().unwrap();
    let mut member = self.data_acc.member.write().unwrap();

    // We are a little wasteful here because we do not insert it directly but rather create a vector first and then copy it over
    for entry in self.db_main.select("SELECT id, nickname, mail, password, salt, xp, mail_confirmed, forgot_password, delete_account, val_prio1, val_prio2, val_prio3, val_hash1, val_hash2, val_hash3 FROM member", &|mut row|{
      Member {
        id: row.take(0).unwrap(),
        nickname: row.take(1).unwrap(),
        mail: row.take(2).unwrap(),
        password: row.take(3).unwrap(),
        salt: row.take(4).unwrap(),
        xp: row.take(5).unwrap(),
        mail_confirmed: row.take(6).unwrap(),
        forgot_password: row.take(7).unwrap(),
        delete_account: row.take(8).unwrap(),
        hash_prio: vec![row.take(9).unwrap(), row.take(10).unwrap(), row.take(11).unwrap()],
        hash_val: vec![row.take(12).unwrap(), row.take(13).unwrap(), row.take(14).unwrap()]
      }
    }) {
      // Chance should be fairly low that we a havea duplicate key
      for i in 0..2 {
        if entry.hash_val[i] != "none" {
          hash_to_member.insert(entry.hash_val[i].clone(), entry.id);
        }
      }

      // Init remaining confirmation mails
      if !entry.mail_confirmed {
        requires_mail_confirmation.insert(hash_sha3(vec![&entry.id.to_string(), &entry.salt]), entry.id);
      }

      // Init remaining forgot password mails
      if entry.forgot_password {
        forgot_password.insert(hash_sha3(vec![&entry.id.to_string(), "forgot", &entry.salt]), entry.id);
      }

      // Init remaining delete mails
      if entry.delete_account {
        delete_account.insert(hash_sha3(vec![&entry.id.to_string(), "delete", &entry.salt]), entry.id);
      }

      member.insert(entry.id, entry);
    }
  }

  fn validate(&self, params: &ValidationPair) -> bool
  {
    let hash_to_member = self.data_acc.hash_to_member.read().unwrap();
    match hash_to_member.get(&params.hash) {
      Some(id) => {
        // Doing it this way, because write locks need to be avoided
        let mut work_key = 3;
        {
          // Updating the prios if necessary
          let member = self.data_acc.member.read().unwrap();
          let entry = member.get(id).unwrap();
          // We need to find the index first
          for i in 0..2 {
            if entry.hash_val[i] == params.hash {
              if entry.hash_prio[i] != 0 {
                work_key = i;
              }
              break;
            }
          }
        }

        if work_key < 3 {
          let mut member = self.data_acc.member.write().unwrap();
          let entry = member.get_mut(id).unwrap();

          // Adjusting prios
          entry.hash_prio[work_key] = 0;
          entry.hash_prio[(work_key+1)%3] += 1;
          entry.hash_prio[(work_key+2)%3] += 1;

          self.db_main.execute_wparams("UPDATE member SET val_prio1=:vp1, val_prio2=:vp2, val_prio3=:vp3 WHERE id=:id", params!(
            "vp1" => entry.hash_prio[0],
            "vp2" => entry.hash_prio[1],
            "vp3" => entry.hash_prio[2],
            "id" => *id
          ));
        }

        *id == params.id
      },
      None => false
    }
  }

  fn get(&self, id: u32) -> Option<AccountInformation>
  {
    let member = self.data_acc.member.read().unwrap();
    match member.get(&id) {
      Some(entry) => Some(AccountInformation {
        mail: entry.mail.clone(),
        xp: entry.xp
      }),
      None => None
    }
  }

  // Helper functions
  fn helper_clear_validation(&self, member_id: &u32, hash_to_member: &mut HashMap<String, u32>, member: &mut HashMap<u32, Member>)
  {
    let entry = member.get_mut(member_id).unwrap();
    for i in 0..2 {
      if entry.hash_val[i] != "none" {
        hash_to_member.remove(&entry.hash_val[i]);
      }
      entry.hash_val[i] = "none".to_string();
      entry.hash_prio[i] = 2;
    }
  }

  fn helper_create_validation(&self, member_id: &u32, hash_to_member: &mut HashMap<String, u32>, member: &mut HashMap<u32, Member>) -> String
  {
    let entry = member.get_mut(member_id).unwrap();

    // Generate a 128 bit salt for our validation hash
    let salt: String = rnd_alphanumeric(16);
    let hash: String = hash_sha3(vec![&entry.mail, &entry.password, &salt]);

    // Replace by using the Least recently used strategy
    for i in 0..2 {
      if entry.hash_prio[i] >= 2 {
        // Adjusting prios
        entry.hash_prio[i] = 0;
        entry.hash_prio[(i+1)%3] += 1;
        entry.hash_prio[(i+2)%3] += 1;

        // Removing previous entry
        hash_to_member.remove(&entry.hash_val[i].clone());
        hash_to_member.insert(hash.clone(), *member_id);
        entry.hash_val[i] = hash.clone();
        break;
      }
    }

    self.db_main.execute_wparams("UPDATE member SET val_hash1=:vh1, val_prio1=:vp1, val_hash2=:vh2, val_prio2=:vp2, val_hash3=:vh3, val_prio3=:vp3 WHERE id=:id", params!(
      "vh1" => entry.hash_val[0].clone(),
      "vp1" => entry.hash_prio[0],
      "vh2" => entry.hash_val[1].clone(),
      "vp2" => entry.hash_prio[1],
      "vh3" => entry.hash_val[2].clone(),
      "vp3" => entry.hash_prio[2],
      "id" => *member_id
    ));

    hash
  }
}