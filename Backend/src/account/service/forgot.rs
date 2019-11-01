use crate::account::material::account::Account;
use crate::account::tools::forgot::Forgot;
use crate::util::language::tools::get::Get;
use crate::util::language::domainvalue::language::Language;

use rocket::response::content;
use rocket::State;

#[get("/forgot/confirm/<id>")]
pub fn receive_confirmation(me: State<Account>, id: String) -> content::Json<String>
{
  match me.recv_forgot_password(&id) {
    Ok(hash) => content::Json(serde_json::to_string(&hash).unwrap()),
    Err(error_str) => content::Json(error_str)
  }
}

#[get("/forgot/send/<mail>")]
pub fn send_confirmation(me: State<Account>, mail: String) -> content::Json<String>
{
  match me.send_forgot_password(&mail) {
    Ok(_) => content::Json(me.dictionary.get("general.service.success", Language::English)),
    Err(error_str) => content::Json(error_str)
  }
}