use rocket::State;
use rocket_contrib::json::Json;

use crate::account::material::{Account, APIToken};
use crate::account::tools::Forgot;

#[get("/forgot/confirm/<id>")]
pub fn receive_confirmation(me: State<Account>, id: String) -> Result<Json<APIToken>, String>
{
  me.recv_forgot_password(&id)
    .and_then(|api_token| Ok(Json(api_token)))
}

#[get("/forgot/send/<mail>")]
pub fn send_confirmation(me: State<Account>, mail: String) -> Result<(), String>
{
  me.send_forgot_password(&mail)
}