use std::sync::RwLock;
use std::collections::HashMap;

use crate::account::material::member::Member;

pub struct AccountData {
  pub member: RwLock<HashMap<u32, Member>>,
  pub hash_to_member: RwLock<HashMap<String, u32>>,
  pub requires_mail_confirmation: RwLock<HashMap<String, u32>>,
  pub forgot_password: RwLock<HashMap<String, u32>>,
  pub delete_account: RwLock<HashMap<String, u32>>,
}
impl AccountData {
  pub fn new() -> Self
  {
    AccountData {
      member: RwLock::new(HashMap::new()),
      hash_to_member: RwLock::new(HashMap::new()),
      requires_mail_confirmation: RwLock::new(HashMap::new()),
      forgot_password: RwLock::new(HashMap::new()),
      delete_account: RwLock::new(HashMap::new()),
    }
  }
}