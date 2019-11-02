use crate::account::material::account::Account;
use crate::account::tools::create::Create;
use crate::account::domainvalue::validation_pair::ValidationPair;
use crate::account::domainvalue::post_create_member::PostCreateMember;

use rocket::State;
use rocket_contrib::json::Json;

#[openapi]
#[post("/create/send", data = "<params>")]
pub fn create(me: State<Account>, params: Json<PostCreateMember>) -> Result<Json<ValidationPair>, String>
{
  match me.create(&params) {
    Ok(val_pair) => Ok(Json(val_pair)),
    Err(error_str) => Err(error_str)
  }
}

#[openapi]
#[get("/create/confirm/<id>")]
pub fn confirm(me: State<Account>, id: String) -> Json<bool>
{
  Json(me.confirm(&id))
}

#[openapi]
#[post("/create/resend", data = "<params>")]
pub fn resend_confirm(me: State<Account>, params: Json<ValidationPair>) -> Json<bool>
{
  Json(me.send_confirmation(&params))
}