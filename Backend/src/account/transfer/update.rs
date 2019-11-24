use rocket::State;
use rocket_contrib::json::Json;

use crate::account::dto::RestrictedContent;
use crate::account::domain_value::{AccountInformation, ValidationPair, Credentials};
use crate::account::material::Account;
use crate::account::tools::{Update, Token, Login};
use language::tools::Get;
use language::domain_value::Language;

#[post("/update/password", data = "<params>")]
pub fn password(me: State<Account>, params: Json<RestrictedContent<String, ValidationPair>>) -> Result<Json<ValidationPair>, String>
{
  if !me.validate_token(&params.validation) {
    return Err(me.dictionary.get("general.error.validate", Language::English));
  }

  match me.change_password(&params.content, params.validation.member_id) {
    Ok(val_pair) => Ok(Json(val_pair)),
    Err(error_str) => Err(error_str)
  }
}

#[post("/update/nickname", data = "<params>")]
pub fn nickname(me: State<Account>, params: Json<RestrictedContent<String, ValidationPair>>) -> Result<Json<AccountInformation>, String>
{
  if !me.validate_token(&params.validation) {
    return Err(me.dictionary.get("general.error.validate", Language::English));
  }

  match me.change_name(&params.content, params.validation.member_id) {
    Ok(acc_info) => Ok(Json(acc_info)),
    Err(error_str) => Err(error_str)
  }
}

#[post("/update/mail", data = "<params>")]
pub fn mail(me: State<Account>, params: Json<RestrictedContent<String, Credentials>>) -> Result<Json<ValidationPair>, String>
{
  me.validate_credentials(&params.validation)
    .and_then(|member_id| me.change_mail(&params.content, member_id))
    .and_then(|val_pair| Ok(Json(val_pair)))
}