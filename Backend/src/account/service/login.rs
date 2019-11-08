use crate::account::material::account::Account;
use crate::account::tools::login::Login;
use crate::account::domainvalue::post_login::PostLogin;
use crate::account::domainvalue::validation_pair::ValidationPair;

use rocket::State;
use rocket_contrib::json::Json;

#[post("/login", data = "<params>")]
pub fn login(me: State<Account>, params: Json<PostLogin>) -> Result<Json<ValidationPair>, String>
{
  match me.login(&params) {
    Ok(val_pair) => Ok(Json(val_pair)),
    Err(error_str) => Err(error_str)
  }
}