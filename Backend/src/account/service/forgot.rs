use language::domain_value::Language;
use language::tools::Get;
use rocket::State;
use rocket_contrib::json::Json;

use crate::account::domain_value::ValidationPair;
use crate::account::material::Account;
use crate::account::tools::Forgot;

#[get("/forgot/confirm/<id>")]
pub fn receive_confirmation(me: State<Account>, id: String) -> Result<Json<ValidationPair>, String>
{
  match me.recv_forgot_password(&id) {
    Ok(val_pair) => Ok(Json(val_pair)),
    Err(error_str) => Err(error_str)
  }
}

#[get("/forgot/send/<mail>")]
pub fn send_confirmation(me: State<Account>, mail: String) -> String
{
  match me.send_forgot_password(&mail) {
    Ok(_) => me.dictionary.get("general.service.success", Language::English),
    Err(error_str) => error_str
  }
}