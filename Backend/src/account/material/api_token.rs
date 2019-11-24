use schemars::JsonSchema;

use crate::account::domain_value::ValidationPair;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct APIToken {
  pub id: u32,
  pub member_id: u32,
  pub token: String,
  pub purpose: String,
  pub exp_date: u64,
}

impl APIToken {
  pub fn to_validation_pair(&self) -> ValidationPair {
    ValidationPair {
      api_token: self.token.clone(),
      member_id: self.member_id,
    }
  }
}