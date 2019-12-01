use rocket::State;
use rocket_contrib::json::Json;

use crate::account::dto::CreateToken;
use crate::account::material::{Account, APIToken};
use crate::account::tools::Token;
use crate::account::guard::authenticate::Authenticate;

#[post("/token/create", data = "<params>")]
pub fn create_token(me: State<Account>, auth: Authenticate, params: Json<CreateToken>) -> Result<Json<APIToken>, String>
{
  me.create_token(&params.purpose, auth.0, params.exp_date)
    .and_then(|api_token| Ok(Json(api_token)))
}

#[get("/token/get")]
pub fn get_tokens(me: State<Account>, auth: Authenticate) -> Result<Json<Vec<APIToken>>, String> {
  Ok(Json(me.get_all_token(auth.0)))
}

#[post("/token/delete", data = "<token_id>")]
pub fn delete_token(me: State<Account>, auth: Authenticate, token_id: Json<u32>) -> Result<(), String>
{
  me.delete_token(token_id.0, auth.0)
}