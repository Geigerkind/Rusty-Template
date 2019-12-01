use language::domain_value::Language;
use language::tools::Get;
use mysql_connection::tools::{Execute, Select};
use str_util::{random, sha3};

use crate::account::material::{Account, APIToken};

pub trait Token {
  fn get_all_token(&self, member_id: u32) -> Vec<APIToken>;
  fn validate_token(&self, api_token: &str) -> Option<u32>;
  fn clear_tokens(&self, member_id: u32) -> Result<(), String>;
  fn create_token(&self, purpose: &str, member_id: u32, exp_date: u64) -> Result<APIToken, String>;
  fn delete_token(&self, token_id: u32, member_id: u32) -> Result<(), String>;
}

impl Token for Account {
  fn get_all_token(&self, member_id: u32) -> Vec<APIToken> {
    let api_tokens = self.api_tokens.read().unwrap();
    match api_tokens.get(&member_id) {
      Some(token_vec) => token_vec.to_vec(),
      None => Vec::new()
    }
  }

  fn validate_token(&self, api_token: &str) -> Option<u32> {
    let api_token_to_member_id = self.api_token_to_member_id.read().unwrap();
    api_token_to_member_id.get(api_token).and_then(|member_id| Some(member_id.clone()))
  }

  fn clear_tokens(&self, member_id: u32) -> Result<(), String> {
    let mut api_token_to_member_id = self.api_token_to_member_id.write().unwrap();
    let mut api_token = self.api_tokens.write().unwrap();

    if !self.db_main.execute_wparams("DELETE FROM api_token WHERE member_id=:member_id", params!(
      "member_id" => member_id
    )) {
      return Err(self.dictionary.get("general.error.unknown", Language::English));
    }

    for api_token in api_token.get(&member_id).unwrap() {
      api_token_to_member_id.remove(&api_token.token);
    }
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

    // The following part needs to be transactional
    let mut api_token_to_member_id = self.api_token_to_member_id.write().unwrap();
    let mut api_tokens = self.api_tokens.write().unwrap();

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

  fn delete_token(&self, token_id: u32, member_id: u32) -> Result<(), String> {
    // We lock before in order to be transactional
    let mut api_token_to_member_id = self.api_token_to_member_id.write().unwrap();
    let mut api_tokens = self.api_tokens.write().unwrap();

    if !self.db_main.execute_wparams(
      "DELETE FROM api_token WHERE id=:id AND member_id=:member_id",
      params!(
        "id" => token_id,
        "member_id" => member_id
      ),
    ) {
      return Err(self.dictionary.get("general.error.unknown", Language::English));
    }

    match api_tokens.get(&member_id).unwrap().iter().position(|api_token| api_token.id == token_id) {
      Some(token_index) => {
        let token_vec = api_tokens.get_mut(&member_id).unwrap();
        {
          let api_token = token_vec.get(token_index).unwrap();
          api_token_to_member_id.remove(&api_token.token);
        }
        token_vec.remove(token_index);
        Ok(())
      },
      None => Err(self.dictionary.get("general.error.unknown", Language::English))
    }
  }
}