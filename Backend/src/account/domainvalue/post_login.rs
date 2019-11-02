use schemars::JsonSchema;
#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct PostLogin{
  pub mail: String,
  pub password: String
}