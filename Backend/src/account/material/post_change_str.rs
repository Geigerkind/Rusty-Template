use crate::account::domainvalue::validation_pair::ValidationPair;
use schemars::JsonSchema;
#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct PostChangeStr {
  pub content: String,
  pub validation: ValidationPair
}