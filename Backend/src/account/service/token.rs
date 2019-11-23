use rocket::State;
use rocket_contrib::json::Json;

use crate::account::domainvalue::post_delete_token::PostDeleteToken;
use crate::account::domainvalue::post_token::PostToken;
use crate::account::domainvalue::validation_pair::ValidationPair;
use crate::account::material::account::Account;
use crate::account::material::api_token::APIToken;
use crate::account::tools::token::Token;

#[post("/token/create", data = "<params>")]
pub fn create_token(me: State<Account>, params: Json<PostToken>) -> Result<Json<APIToken>, String>
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
pub fn delete_token(me: State<Account>, params: Json<PostDeleteToken>) -> Result<(), String>
{
  me.delete_token(&params)
}