use rocket::State;
use rocket_contrib::json::Json;

use crate::account::domain_value::{Credentials, ValidationPair};
use crate::account::material::Account;
use crate::account::tools::Login;

#[post("/login", data = "<params>")]
pub fn login(me: State<Account>, params: Json<Credentials>) -> Result<Json<ValidationPair>, String>
{
  match me.login(&params) {
    Ok(val_pair) => Ok(Json(val_pair)),
    Err(error_str) => Err(error_str)
  }
}