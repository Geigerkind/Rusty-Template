use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct CreateMember {
  pub nickname: String,
  pub mail: String,
  pub password: String,
}