use crate::account::material::account::Account;
use crate::account::tools::update::Update;
use crate::account::domainvalue::validation_pair::ValidationPair;

use rocket::response::content;
use rocket::State;
use rocket_contrib::json::Json;

#[derive(Deserialize)]
pub struct PostChangeStr {
  pub content: String,
  pub validation: ValidationPair
}
#[post("/update/password", data = "<params>")]
pub fn password(me: State<Account>, params: Json<PostChangeStr>) -> content::Json<String>
{
  content::Json(serde_json::to_string(&me.change_password(&params).unwrap()).unwrap())
}
#[post("/update/nickname", data = "<params>")]
pub fn nickname(me: State<Account>, params: Json<PostChangeStr>) -> content::Json<String>
{
  content::Json(me.change_name(&params).to_string())
}
#[post("/update/mail", data = "<params>")]
pub fn mail(me: State<Account>, params: Json<PostChangeStr>) -> content::Json<String>
{
  content::Json(serde_json::to_string(&me.change_mail(&params).unwrap()).unwrap())
}