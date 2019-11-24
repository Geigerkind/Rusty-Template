use schemars::JsonSchema;

use crate::account::domain_value::ValidationPair;

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct UpdateContent {
  pub content: String,
  pub validation: ValidationPair,
}