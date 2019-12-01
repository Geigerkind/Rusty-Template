use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct Credentials {
  pub mail: String,
  pub password: String,
}