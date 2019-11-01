use crate::account::material::account::Account;
use crate::account::tools::login::Login;
use crate::account::domainvalue::post_login::PostLogin;

use rocket::response::content;
use rocket::State;
use rocket_contrib::json::Json;

#[post("/login", data = "<params>")]
pub fn login(me: State<Account>, params: Json<PostLogin>) -> content::Json<String>
{
  match me.login(&params) {
    Ok(hash) => content::Json(serde_json::to_string(&hash).unwrap()),
    Err(error_str) => content::Json(error_str)
  }
}