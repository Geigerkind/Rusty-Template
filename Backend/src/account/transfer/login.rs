use rocket::State;
use rocket_contrib::json::Json;

use crate::account::domain_value::{Credentials, ValidationPair};
use crate::account::material::Account;
use crate::account::tools::Login;

#[post("/login", data = "<params>")]
pub fn login(me: State<Account>, params: Json<Credentials>) -> Result<Json<ValidationPair>, String>
{
  me.login(&params)
    .and_then(|val_pair| Ok(Json(val_pair)))
}