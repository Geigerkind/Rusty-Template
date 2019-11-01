use crate::database::material::mysql_connection::MySQLConnection;
use crate::account::material::member::Member;
use crate::util::str_util::tools::sha3;
use crate::database::tools::mysql::select::Select;

use std::collections::HashMap;
use std::sync::RwLock;
use crate::util::language::material::dictionary::Dictionary;
use crate::account::language::init::Init;

pub struct Account {
  pub db_main: MySQLConnection,
  pub dictionary: Dictionary,
  pub member: RwLock<HashMap<u32, Member>>,
  pub hash_to_member: RwLock<HashMap<String, u32>>,
  pub requires_mail_confirmation: RwLock<HashMap<String, u32>>,
  pub forgot_password: RwLock<HashMap<String, u32>>,
  pub delete_account: RwLock<HashMap<String, u32>>,
}

impl Default for Account {
  fn default() -> Self
  {
    let dictionary = Dictionary::default();
    Dictionary::init(&dictionary);
    Account {
      db_main: MySQLConnection::new("main"),
      dictionary,
      member: RwLock::new(HashMap::new()),
      hash_to_member: RwLock::new(HashMap::new()),
      requires_mail_confirmation: RwLock::new(HashMap::new()),
      forgot_password: RwLock::new(HashMap::new()),
      delete_account: RwLock::new(HashMap::new()),
    }
  }
}

impl Account {
  pub fn init(&self)
  {
    let mut requires_mail_confirmation = self.requires_mail_confirmation.write().unwrap();
    let mut forgot_password = self.forgot_password.write().unwrap();
    let mut delete_account = self.delete_account.write().unwrap();
    let mut hash_to_member = self.hash_to_member.write().unwrap();
    let mut member = self.member.write().unwrap();

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
        requires_mail_confirmation.insert(sha3::hash(&[&entry.id.to_string(), &entry.salt]), entry.id);
      }

      // Init remaining forgot password mails
      if entry.forgot_password {
        forgot_password.insert(sha3::hash(&[&entry.id.to_string(), "forgot", &entry.salt]), entry.id);
      }

      // Init remaining delete mails
      if entry.delete_account {
        delete_account.insert(sha3::hash(&[&entry.id.to_string(), "delete", &entry.salt]), entry.id);
      }

      member.insert(entry.id, entry);
    }
  }
}