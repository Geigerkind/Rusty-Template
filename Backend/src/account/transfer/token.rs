use rocket::State;
use rocket_contrib::json::Json;

use crate::account::domain_value::{DeleteToken, CreateToken, ValidationPair};
use crate::account::material::{Account, APIToken};
use crate::account::tools::Token;
use language::domain_value::Language;
use language::tools::Get;

#[post("/token/create", data = "<params>")]
pub fn create_token(me: State<Account>, params: Json<CreateToken>) -> Result<Json<APIToken>, String>
{
  if !me.validate_token(&params.val_pair) {
    return Err(me.dictionary.get("general.error.validate", Language::English));
  }

  match me.create_token(&params.purpose, params.val_pair.member_id, params.exp_date) {
    Ok(token) => Ok(Json(token)),
    Err(err_str) => Err(err_str)
  }
}

#[post("/token/get", data = "<params>")]
pub fn get_tokens(me: State<Account>, params: Json<ValidationPair>) -> Result<Json<Vec<APIToken>>, String> {
  if !me.validate_token(&params) {
    return Err(me.dictionary.get("general.error.validate", Language::English));
  }

  Ok(Json(me.get_all_token(params.member_id)))
}

#[post("/token/delete", data = "<params>")]
pub fn delete_token(me: State<Account>, params: Json<DeleteToken>) -> Result<(), String>
{
  if !me.validate_token(&params.val_pair) {
    return Err(me.dictionary.get("general.error.validate", Language::English));
  }
  me.delete_token(params.token_id, params.val_pair.member_id)
}