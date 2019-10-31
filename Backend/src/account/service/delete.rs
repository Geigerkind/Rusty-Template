use crate::account::material::account::Account;
use crate::account::tools::delete::Delete;
use crate::account::domainvalue::validation_pair::ValidationPair;
use crate::language::tools::get::Get;
use crate::language::domainvalue::language::Language;

use rocket::response::content;
use rocket::State;
use rocket_contrib::json::Json;

#[get("/delete/confirm/<id>")]
pub fn confirm(me: State<Account>, id: String) -> content::Json<String>
{
  match me.confirm_delete(&id) {
    Ok(_) => content::Json(me.dictionary.get("general.service.success", Language::English)),
    Err(error_str) => content::Json(error_str)
  }
}

#[delete("/delete/send", data = "<params>")]
pub fn request(me: State<Account>, params: Json<ValidationPair>) -> content::Json<String>
{
  match me.issue_delete(&params) {
    Ok(acc_info) => content::Json(serde_json::to_string(&acc_info).unwrap()),
    Err(error_str) => content::Json(error_str)
  }
}

