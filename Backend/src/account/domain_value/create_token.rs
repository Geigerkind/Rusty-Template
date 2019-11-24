use schemars::JsonSchema;

use crate::account::domain_value::ValidationPair;

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct CreateToken {
  pub purpose: String,
  pub exp_date: u64,
  pub val_pair: ValidationPair,
}