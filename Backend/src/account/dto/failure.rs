use schemars::JsonSchema;

#[derive(Responder, Debug, JsonSchema)]
pub enum Failure {
  #[response(status = 599, content_type = "text/plain")]
  Login(String)
}