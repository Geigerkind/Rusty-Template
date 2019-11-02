use schemars::JsonSchema;
#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct PostCreateMember{
  pub nickname: String,
  pub mail: String,
  pub password: String
}