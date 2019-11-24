use crate::account::material::Account;
use crate::account::tools::Create;
use crate::account::domain_value::ValidationPair;
use crate::account::domain_value::CreateMember;

use rocket::State;
use rocket_contrib::json::Json;

#[post("/create/send", data = "<params>")]
pub fn create(me: State<Account>, params: Json<CreateMember>) -> Result<Json<ValidationPair>, String>
{
  match me.create(&params) {
    Ok(val_pair) => Ok(Json(val_pair)),
    Err(error_str) => Err(error_str)
  }
}

#[get("/create/confirm/<id>")]
pub fn confirm(me: State<Account>, id: String) -> Json<bool>
{
  Json(me.confirm(&id))
}

#[post("/create/resend", data = "<params>")]
pub fn resend_confirm(me: State<Account>, params: Json<ValidationPair>) -> Json<bool>
{
  Json(me.send_confirmation(&params))
}