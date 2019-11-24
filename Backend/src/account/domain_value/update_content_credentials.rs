use schemars::JsonSchema;

use crate::account::domain_value::Credentials;

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct UpdateContentCredentials {
  pub content: String,
  pub credentials: Credentials,
}