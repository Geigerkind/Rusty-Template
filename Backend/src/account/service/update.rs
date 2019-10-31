use crate::account::material::account::Account;
use crate::account::tools::update::Update;
use crate::account::domainvalue::validation_pair::ValidationPair;

use rocket::response::content;
use rocket::State;
use rocket_contrib::json::Json;

#[derive(Deserialize)]
pub struct PostChangeStr {
  pub content: String,
  pub validation: ValidationPair
}
#[post("/update/password", data = "<params>")]
pub fn password(me: State<Account>, params: Json<PostChangeStr>) -> content::Json<String>
{
  match me.change_password(&params) {
    Ok(val_pair) => content::Json(serde_json::to_string(&val_pair).unwrap()),
    Err(error_str) => content::Json(error_str)
  }
}
#[post("/update/nickname", data = "<params>")]
pub fn nickname(me: State<Account>, params: Json<PostChangeStr>) -> content::Json<String>
{
  match me.change_name(&params) {
    Ok(acc_info) => content::Json(serde_json::to_string(&acc_info).unwrap()),
    Err(error_str) => content::Json(error_str)
  }
}
#[post("/update/mail", data = "<params>")]
pub fn mail(me: State<Account>, params: Json<PostChangeStr>) -> content::Json<String>
{
  match me.change_mail(&params) {
    Ok(val_pair) => content::Json(serde_json::to_string(&val_pair).unwrap()),
    Err(error_str) => content::Json(error_str)
  }
}