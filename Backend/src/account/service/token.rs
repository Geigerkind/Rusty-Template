use rocket::State;
use rocket_contrib::json::Json;

use crate::account::domain_value::{DeleteToken, CreateToken, ValidationPair};
use crate::account::material::{Account, APIToken};
use crate::account::tools::Token;

#[post("/token/create", data = "<params>")]
pub fn create_token(me: State<Account>, params: Json<CreateToken>) -> Result<Json<APIToken>, String>
{
  match me.create_token(&params) {
    Ok(token) => Ok(Json(token)),
    Err(err_str) => Err(err_str)
  }
}

#[post("/token/get", data = "<params>")]
pub fn get_tokens(me: State<Account>, params: Json<ValidationPair>) -> Result<Json<Vec<APIToken>>, String> {
  match me.get_all_token(&params) {
    Ok(tokens) => Ok(Json(tokens)),
    Err(err_str) => Err(err_str)
  }
}

#[post("/token/delete", data = "<params>")]
pub fn delete_token(me: State<Account>, params: Json<DeleteToken>) -> Result<(), String>
{
  me.delete_token(&params)
}