use schemars::JsonSchema;
#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct Member {
  pub id: u32,
  pub nickname: String,
  pub mail: String,
  pub password: String,
  pub salt: String,
  pub xp: u32,
  pub mail_confirmed: bool,
  pub forgot_password: bool,
  pub delete_account: bool,
  pub hash_prio: Vec<u8>,
  pub hash_val: Vec<String>
}