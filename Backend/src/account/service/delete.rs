use crate::account::material::account::Account;
use crate::account::tools::delete::Delete;
use crate::account::domainvalue::validation_pair::ValidationPair;
use language::get::Get;
use language::domainvalue::language::Language;
use crate::account::domainvalue::account_information::AccountInformation;

use rocket::State;
use rocket_contrib::json::Json;

#[get("/delete/confirm/<id>")]
pub fn confirm(me: State<Account>, id: String) -> String
{
  match me.confirm_delete(&id) {
    Ok(_) => me.dictionary.get("general.service.success", Language::English),
    Err(error_str) => error_str
  }
}

#[delete("/delete/send", data = "<params>")]
pub fn request(me: State<Account>, params: Json<ValidationPair>) -> Result<Json<AccountInformation>, String>
{
  match me.issue_delete(&params) {
    Ok(acc_info) => Ok(Json(acc_info)),
    Err(error_str) => Err(error_str)
  }
}

