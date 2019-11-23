use crate::account::domainvalue::validation_pair::ValidationPair;

use schemars::JsonSchema;
#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct PostToken{
  pub purpose: String,
  pub exp_date: u64,
  pub val_pair: ValidationPair
}