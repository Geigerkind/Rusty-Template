use language::domainvalue::Language;
use language::tools::Get;
use mysql_connection::tools::{ Execute, Select };
use str_util::{random, sha3};

use crate::account::domainvalue::post_delete_token::PostDeleteToken;
use crate::account::domainvalue::post_token::PostToken;
use crate::account::domainvalue::validation_pair::ValidationPair;
use crate::account::material::account::Account;
use crate::account::material::api_token::APIToken;

pub trait Token {
  fn get_all_token(&self, params: &ValidationPair) -> Result<Vec<APIToken>, String>;
  fn validate(&self, params: &ValidationPair) -> bool;
  fn clear_tokens(&self, member_id: u32) -> Result<(), String>;
  fn create_token(&self, params: &PostToken) -> Result<APIToken, String>;
  fn create_token_unsafe(&self, purpose: &str, member_id: u32, exp_date: u64) -> Result<APIToken, String>;
  fn create_validation_unsafe(&self, purpose: &str, member_id: u32, exp_date: u64) -> Result<ValidationPair, String>;
  fn delete_token(&self, params: &PostDeleteToken) -> Result<(), String>;
}

impl Token for Account {
  fn get_all_token(&self, params: &ValidationPair) -> Result<Vec<APIToken>, String> {
    if !self.validate(&params) {
      return Err(self.dictionary.get("general.error.validate", Language::English));
    }

    let api_tokens = self.api_token.read().unwrap();
    match api_tokens.get(&params.member_id) {
      Some(token_vec) => Ok(token_vec.to_vec()),
      None => Ok(vec![])
    }
  }

  fn validate(&self, params: &ValidationPair) -> bool {
    let api_token_to_member_id = self.api_token_to_member_id.read().unwrap();
    match api_token_to_member_id.get(&params.api_token) {
      Some(member_id) => {
        let api_tokens = self.api_token.read().unwrap();
        match api_tokens.get(&member_id) {
          Some(token_vec) => {
            for token in token_vec {
              if token.token == params.api_token {
                return true;
              }
            }
            false
          }
          None => return false
        }
      }
      None => return false
    }
  }

  fn clear_tokens(&self, member_id: u32) -> Result<(), String> {
    {
      // Checking for existance
      let member = self.member.read().unwrap();
      if member.get(&member_id).is_none() {
        return Err(self.dictionary.get("validator.error.member_exist", Language::English));
      }
    }

    // First attempting to remove entries from the DB
    if !self.db_main.execute_wparams("DELETE FROM api_token WHERE member_id=:member_id", params!(
      "member_id" => member_id
    )) {
      return Err(self.dictionary.get("general.error.unknown", Language::English));
    }

    let mut api_token_to_member_id = self.api_token_to_member_id.write().unwrap();
    let mut api_token = self.api_token.write().unwrap();

    // Next clearing internal data structures
    for token in api_token.get(&member_id).unwrap() {
      api_token_to_member_id.remove(&token.token);
    }

    api_token.get_mut(&member_id).unwrap().clear();

    Ok(())
  }

  fn create_token(&self, params: &PostToken) -> Result<APIToken, String> {
    if !self.validate(&params.val_pair) {
      return Err(self.dictionary.get("general.error.validate", Language::English));
    }

    self.create_token_unsafe(&params.purpose, params.val_pair.member_id, params.exp_date)
  }

  fn create_token_unsafe(&self, purpose: &str, member_id: u32, exp_date: u64) -> Result<APIToken, String> {
    let token: String;
    {
      let member = self.member.read().unwrap();
      let member_entry = member.get(&member_id).unwrap();
      let salt: String = random::alphanumeric(16);
      token = sha3::hash(&[&member_entry.mail, &member_entry.password, &salt]);
    }

    if !self.db_main.execute_wparams(
      "INSERT INTO api_token (member_id, token, purpose, exp_date) VALUES (:member_id, :token, :purpose, :exp_date)",
      params!(
        "member_id" => member_id,
        "token" => token.clone(),
        "purpose" => purpose,
        "exp_date" => exp_date
      ),
    ) {
      return Err(self.dictionary.get("general.error.unknown", Language::English));
    }

    match self.db_main.select_wparams_value(
      "SELECT id, member_id, token, purpose, exp_date FROM api_token WHERE member_id=:member_id AND token=:token",
      &|mut row| {
        APIToken {
          id: row.take(0).unwrap(),
          member_id: row.take(1).unwrap(),
          token: row.take(2).unwrap(),
          purpose: row.take(3).unwrap(),
          exp_date: row.take(4).unwrap(),
        }
      },
      params!(
        "member_id" => member_id,
        "token" => token.clone()
      ),
    ) {
      Some(token) => {
        let mut api_token_to_member_id = self.api_token_to_member_id.write().unwrap();
        let mut api_tokens = self.api_token.write().unwrap();
        if api_tokens.get(&member_id).is_none() {
          api_tokens.insert(member_id, vec![token.clone()]);
        } else {
          api_tokens.get_mut(&member_id).unwrap().push(token.clone());
        }
        api_token_to_member_id.insert(token.token.clone(), member_id);
        Ok(token)
      }
      None => return Err(self.dictionary.get("general.error.unknown", Language::English))
    }
  }

  fn create_validation_unsafe(&self, purpose: &str, member_id: u32, exp_date: u64) -> Result<ValidationPair, String> {
    match self.create_token_unsafe(purpose, member_id, exp_date) {
      Ok(api_token) => Ok(api_token.to_validation_pair()),
      Err(err_str) => Err(err_str)
    }
  }

  fn delete_token(&self, params: &PostDeleteToken) -> Result<(), String> {
    if !self.validate(&params.val_pair) {
      return Err(self.dictionary.get("general.error.validate", Language::English));
    }

    if !self.db_main.execute_wparams(
      "DELETE FROM api_token WHERE id=:id AND member_id=:member_id",
      params!(
        "id" => params.token_id,
        "member_id" => params.val_pair.member_id
      ),
    ) {
      return Err(self.dictionary.get("general.error.unknown", Language::English));
    }

    let mut api_token_to_member_id = self.api_token_to_member_id.write().unwrap();
    let mut api_tokens = self.api_token.write().unwrap();

    let mut token: String = String::from("");
    let mut token_index: usize = 0;
    for api_token in api_tokens.get(&params.val_pair.member_id).unwrap() {
      if api_token.id == params.token_id {
        token = api_token.token.clone();
        break;
      }
      token_index = token_index + 1;
    }

    if token.is_empty() {
      return Err(self.dictionary.get("general.error.unknown", Language::English));
    }

    api_token_to_member_id.remove(&token);
    api_tokens.get_mut(&params.val_pair.member_id).unwrap().remove(token_index);

    Ok(())
  }
}