use schemars::JsonSchema;
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct AccountInformation {
  pub id: u32,
  pub mail: String,
  pub nickname: String,
  pub mail_confirmed: bool,
  pub xp: u32
}