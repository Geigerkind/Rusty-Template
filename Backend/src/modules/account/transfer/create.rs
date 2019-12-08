use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::account::dto::{CreateMember, Failure};
use crate::modules::account::material::{Account, APIToken};
use crate::modules::account::tools::Create;
use crate::modules::account::guard::Authenticate;

#[post("/create", data = "<params>")]
pub fn create(me: State<Account>, params: Json<CreateMember>) -> Result<Json<APIToken>, Failure>
{
  me.create(&params.credentials.mail, &params.nickname, &params.credentials.password)
    .and_then(|api_token| Ok(Json(api_token)))
}

#[get("/create/confirm/<id>")]
pub fn confirm(me: State<Account>, id: String) -> Json<bool>
{
  Json(me.confirm(&id))
}

#[get("/create/resend")]
pub fn resend_confirm(me: State<Account>, auth: Authenticate) -> Json<bool>
{
  Json(me.send_confirmation(auth.0))
}