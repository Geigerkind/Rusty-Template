use crate::util::sha3;
use crate::util::random;
use crate::account::material::account::Account;
use crate::account::domainvalue::validation_pair::ValidationPair;
use crate::account::material::member::Member;
use crate::database::tools::mysql::execute::Execute;

use std::collections::HashMap;

pub trait Validator {
  fn validate(&self, params: &ValidationPair) -> bool;

  fn helper_clear_validation(&self, member_id: u32, hash_to_member: &mut HashMap<String, u32>, member: &mut HashMap<u32, Member>);
  fn helper_create_validation(&self, member_id: u32, hash_to_member: &mut HashMap<String, u32>, member: &mut HashMap<u32, Member>) -> String;
}

impl Validator for Account {
  fn validate(&self, params: &ValidationPair) -> bool
  {
    let hash_to_member = self.hash_to_member.read().unwrap();
    match hash_to_member.get(&params.hash) {
      Some(id) => {
        // Doing it this way, because write locks need to be avoided
        let mut work_key = 3;
        {
          // Updating the prios if necessary
          let member = self.member.read().unwrap();
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
          let mut member = self.member.write().unwrap();
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

  // Helper functions
  fn helper_clear_validation(&self, member_id: u32, hash_to_member: &mut HashMap<String, u32>, member: &mut HashMap<u32, Member>)
  {
    let entry = member.get_mut(&member_id).unwrap();
    for i in 0..2 {
      if entry.hash_val[i] != "none" {
        hash_to_member.remove(&entry.hash_val[i]);
      }
      entry.hash_val[i] = "none".to_string();
      entry.hash_prio[i] = 2;
    }
  }

  fn helper_create_validation(&self, member_id: u32, hash_to_member: &mut HashMap<String, u32>, member: &mut HashMap<u32, Member>) -> String
  {
    let entry = member.get_mut(&member_id).unwrap();

    // Generate a 128 bit salt for our validation hash
    let salt: String = random::alphanumeric(16);
    let hash: String = sha3::hash(vec![&entry.mail, &entry.password, &salt]);

    // Replace by using the Least recently used strategy
    for i in 0..2 {
      if entry.hash_prio[i] >= 2 {
        // Adjusting prios
        entry.hash_prio[i] = 0;
        entry.hash_prio[(i+1)%3] += 1;
        entry.hash_prio[(i+2)%3] += 1;

        // Removing previous entry
        hash_to_member.remove(&entry.hash_val[i].clone());
        hash_to_member.insert(hash.clone(), member_id);
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
      "id" => member_id
    ));

    hash
  }
}