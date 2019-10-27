use crate::account::material::account::Account;
use crate::account::tools::delete::Delete;
use crate::account::domainvalue::validation_pair::ValidationPair;

use rocket::response::content;
use rocket::State;
use rocket_contrib::json::Json;

#[get("/delete/confirm/<id>")]
pub fn confirm(me: State<Account>, id: String) -> content::Json<String>
{
  content::Json(me.confirm_delete(&id).to_string())
}

#[delete("/delete/send", data = "<params>")]
pub fn request(me: State<Account>, params: Json<ValidationPair>) -> content::Json<String>
{
  content::Json(me.issue_delete(&params).to_string())
}

