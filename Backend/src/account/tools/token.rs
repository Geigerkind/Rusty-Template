use language::domain_value::Language;
use language::tools::Get;
use mysql_connection::tools::{Execute, Select};
use str_util::{random, sha3};

use crate::account::domain_value::ValidationPair;
use crate::account::material::{Account, APIToken};

pub trait Token {
  fn get_all_token(&self, member_id: u32) -> Vec<APIToken>;
  fn validate_token(&self, params: &ValidationPair) -> bool;
  fn clear_tokens(&self, member_id: u32) -> Result<(), String>;
  fn create_token(&self, purpose: &str, member_id: u32, exp_date: u64) -> Result<APIToken, String>;
  fn create_validation(&self, purpose: &str, member_id: u32, exp_date: u64) -> Result<ValidationPair, String>;
  fn delete_token(&self, token_id: u32, member_id: u32) -> Result<(), String>;
}

impl Token for Account {
  fn get_all_token(&self, member_id: u32) -> Vec<APIToken> {
    let api_tokens = self.api_tokens.read().unwrap();
    match api_tokens.get(&member_id) {
      Some(token_vec) => token_vec.to_vec(),
      None => vec![]
    }
  }

  fn validate_token(&self, params: &ValidationPair) -> bool {
    let api_tokens = self.api_tokens.read().unwrap();
    match api_tokens.get(&params.member_id) {
      Some(token_vec) => {
        for token in token_vec {
          if token.token == params.api_token {
            return true;
          }
        }
        false
      },
      None => false
    }
  }

  fn clear_tokens(&self, member_id: u32) -> Result<(), String> {
    {
      // Checking for existence
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

    let mut api_token = self.api_tokens.write().unwrap();
    api_token.get_mut(&member_id).unwrap().clear();

    Ok(())
  }

  fn create_token(&self, purpose: &str, member_id: u32, exp_date: u64) -> Result<APIToken, String> {
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
        let mut api_tokens = self.api_tokens.write().unwrap();
        if api_tokens.get(&member_id).is_none() {
          api_tokens.insert(member_id, vec![token.clone()]);
        } else {
          api_tokens.get_mut(&member_id).unwrap().push(token.clone());
        }
        Ok(token)
      }
      None => return Err(self.dictionary.get("general.error.unknown", Language::English))
    }
  }

  fn create_validation(&self, purpose: &str, member_id: u32, exp_date: u64) -> Result<ValidationPair, String> {
    match self.create_token(purpose, member_id, exp_date) {
      Ok(api_token) => Ok(api_token.to_validation_pair()),
      Err(err_str) => Err(err_str)
    }
  }

  fn delete_token(&self, token_id: u32, member_id: u32) -> Result<(), String> {
    if !self.db_main.execute_wparams(
      "DELETE FROM api_token WHERE id=:id AND member_id=:member_id",
      params!(
        "id" => token_id,
        "member_id" => member_id
      ),
    ) {
      return Err(self.dictionary.get("general.error.unknown", Language::English));
    }

    let mut api_tokens = self.api_tokens.write().unwrap();

    let mut token: String = String::from("");
    let mut token_index: usize = 0;
    for api_token in api_tokens.get(&member_id).unwrap() {
      if api_token.id == token_id {
        token = api_token.token.clone();
        break;
      }
      token_index = token_index + 1;
    }

    if token.is_empty() {
      return Err(self.dictionary.get("general.error.unknown", Language::English));
    }

    api_tokens.get_mut(&member_id).unwrap().remove(token_index);

    Ok(())
  }
}