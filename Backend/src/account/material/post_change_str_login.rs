use crate::account::domainvalue::post_login::PostLogin;
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct PostChangeStrLogin {
  pub content: String,
  pub credentials: PostLogin
}