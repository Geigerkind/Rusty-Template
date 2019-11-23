use crate::account::domainvalue::validation_pair::ValidationPair;

use schemars::JsonSchema;
#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct PostDeleteToken{
  pub token_id: u32,
  pub val_pair: ValidationPair
}