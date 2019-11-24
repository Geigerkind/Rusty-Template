use language::domain_value::Language;
use language::tools::Get;
use rocket::State;
use rocket_contrib::json::Json;

use crate::account::domain_value::{AccountInformation, ValidationPair};
use crate::account::material::Account;
use crate::account::tools::{Delete, Token};

#[get("/delete/confirm/<id>")]
pub fn confirm(me: State<Account>, id: String) -> String
{
  match me.confirm_delete(&id) {
    Ok(_) => me.dictionary.get("general.service.success", Language::English),
    Err(error_str) => error_str
  }
}

#[delete("/delete/send", data = "<params>")]
pub fn request(me: State<Account>, params: Json<ValidationPair>) -> Result<Json<AccountInformation>, String>
{
  if !me.validate(&params) {
    return Err(me.dictionary.get("general.error.validate", Language::English));
  }

  match me.issue_delete(params.member_id) {
    Ok(acc_info) => Ok(Json(acc_info)),
    Err(error_str) => Err(error_str)
  }
}

