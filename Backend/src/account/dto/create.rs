use crate::account::material::account::Account;
use crate::account::tools::create::Create;
use crate::account::domainvalue::validation_pair::ValidationPair;

use rocket::response::content;
use rocket::State;
use rocket_contrib::json::Json;

#[derive(Deserialize)]
pub struct PostCreateMember{
  pub nickname: String,
  pub mail: String,
  pub password: String
}
#[post("/create/send", data = "<params>")]
pub fn create(me: State<Account>, params: Json<PostCreateMember>) -> content::Json<String>
{
  content::Json(me.create(&params).to_string())
}

#[get("/create/confirm/<id>")]
pub fn confirm(me: State<Account>, id: String) -> content::Json<String>
{
  content::Json(me.confirm(&id).to_string())
}

#[post("/create/resend", data = "<params>")]
pub fn resend_confirm(me: State<Account>, params: Json<ValidationPair>) -> content::Json<String>
{
  content::Json(me.send_confirmation(&params, false).to_string())
}