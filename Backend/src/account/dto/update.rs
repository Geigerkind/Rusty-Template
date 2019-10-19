use crate::Backend;
use crate::account::tools::update::AccountUpdate;
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
pub fn password(me: State<Backend>, params: Json<PostChangeStr>) -> content::Json<String>
{
  content::Json(me.change_password(&params).unwrap())
}
#[post("/update/nickname", data = "<params>")]
pub fn nickname(me: State<Backend>, params: Json<PostChangeStr>) -> content::Json<String>
{
  content::Json(me.change_name(&params).to_string())
}
#[post("/update/mail", data = "<params>")]
pub fn mail(me: State<Backend>, params: Json<PostChangeStr>) -> content::Json<String>
{
  content::Json(me.change_mail(&params).unwrap())
}