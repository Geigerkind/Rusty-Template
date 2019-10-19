use crate::Backend;
use crate::account::api::Account;
use crate::account::domainvalue::validation_pair::ValidationPair;

use rocket::response::content;
use rocket::State;
use rocket_contrib::json::Json;

#[get("/forgot/confirm/<id>")]
pub fn receive_confirmation(me: State<Backend>, id: String) -> content::Json<String>
{
    content::Json(me.recv_forgot_password(&id).to_string())
}

#[post("/forgot/send", data = "<params>")]
pub fn send_confirmation(me: State<Backend>, params: Json<ValidationPair>) -> content::Json<String>
{
    content::Json(me.send_forgot_password(&params).to_string())
}