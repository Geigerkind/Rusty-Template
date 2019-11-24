use schemars::JsonSchema;

use crate::account::domain_value::PostLogin;

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct PostChangeStrLogin {
  pub content: String,
  pub credentials: PostLogin,
}