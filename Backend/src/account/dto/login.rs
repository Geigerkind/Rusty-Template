use crate::account::material::account::Account;
use crate::account::tools::login::Login;

use rocket::response::content;
use rocket::State;
use rocket_contrib::json::Json;

#[derive(Deserialize)]
pub struct PostLogin{
  pub mail: String,
  pub password: String
}

#[post("/login", data = "<params>")]
pub fn login(me: State<Account>, params: Json<PostLogin>) -> content::Json<String>
{
  match me.login(&params) {
    Some(hash) => content::Json(hash),
    None => content::Json("Error?!".to_string()) // 404 ?
  }
}