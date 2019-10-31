use crate::account::material::account::Account;
use crate::account::tools::forgot::Forgot;
use crate::account::domainvalue::validation_pair::ValidationPair;
use crate::util::language::tools::get::Get;
use crate::util::language::domainvalue::language::Language;

use rocket::response::content;
use rocket::State;
use rocket_contrib::json::Json;

#[get("/forgot/confirm/<id>")]
pub fn receive_confirmation(me: State<Account>, id: String) -> content::Json<String>
{
  match me.recv_forgot_password(&id) {
    Ok(_) => content::Json(me.dictionary.get("general.service.success", Language::English)),
    Err(error_str) => content::Json(error_str)
  }
}

#[post("/forgot/send", data = "<params>")]
pub fn send_confirmation(me: State<Account>, params: Json<ValidationPair>) -> content::Json<String>
{
  match me.send_forgot_password(&params) {
    Ok(_) => content::Json(me.dictionary.get("general.service.success", Language::English)),
    Err(error_str) => content::Json(error_str)
  }
}