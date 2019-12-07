use rocket::State;
use rocket_contrib::json::Json;

use crate::account::material::{Account, APIToken};
use crate::account::tools::Login;
use crate::account::dto::{Credentials, Failure};

#[post("/login", format="application/json", data = "<params>")]
pub fn login(me: State<Account>, params: Json<Credentials>) -> Result<Json<APIToken>, Failure>
{
  me.login(&params.mail, &params.password)
    .and_then(|api_token| Ok(Json(api_token)))
}