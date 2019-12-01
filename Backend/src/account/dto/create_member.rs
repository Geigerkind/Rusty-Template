use schemars::JsonSchema;
use crate::account::dto::Credentials;

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct CreateMember {
  pub nickname: String,
  pub credentials: Credentials
}