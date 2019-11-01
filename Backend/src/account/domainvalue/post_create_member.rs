#[derive(Deserialize)]
pub struct PostCreateMember{
  pub nickname: String,
  pub mail: String,
  pub password: String
}