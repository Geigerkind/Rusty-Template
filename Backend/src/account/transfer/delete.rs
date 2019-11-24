use language::domain_value::Language;
use language::tools::Get;
use rocket::State;
use rocket_contrib::json::Json;

use crate::account::domain_value::{AccountInformation, ValidationPair};
use crate::account::material::Account;
use crate::account::tools::{Delete, Token};

#[get("/delete/confirm/<id>")]
pub fn confirm(me: State<Account>, id: String) -> Result<String, String>
{
  me.confirm_delete(&id)
    .and_then(|()| Ok(me.dictionary.get("general.service.success", Language::English)))
}

#[delete("/delete/send", data = "<params>")]
pub fn request(me: State<Account>, params: Json<ValidationPair>) -> Result<Json<AccountInformation>, String>
{
  if !me.validate_token(&params) {
    return Err(me.dictionary.get("general.error.validate", Language::English));
  }

  me.issue_delete(params.member_id)
    .and_then(|acc_info| Ok(Json(acc_info)))
}

