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
  me.recv_forgot_password(&id)
    .and_then(|val_pair| Ok(Json(val_pair)))
}

#[get("/forgot/send/<mail>")]
pub fn send_confirmation(me: State<Account>, mail: String) -> Result<String, String>
{
  me.send_forgot_password(&mail)
    .and_then(|()| Ok(me.dictionary.get("general.service.success", Language::English)))
}