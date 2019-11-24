use rocket::State;
use rocket_contrib::json::Json;

use crate::account::domain_value::{AccountInformation, ValidationPair};
use crate::account::material::Account;
use crate::account::tools::{GetAccountInformation, Token};
use language::tools::Get;
use language::domain_value::Language;

#[post("/get", data="<params>")]
pub fn get_account_information(me: State<Account>, params: Json<ValidationPair>) -> Result<Json<AccountInformation>, String>
{
  if !me.validate(&params) {
    return Err(me.dictionary.get("general.error.validate", Language::English));
  }

  match me.get(params.member_id) {
    Ok(acc_info) => Ok(Json(acc_info)),
    Err(err_str) => Err(err_str)
  }
}