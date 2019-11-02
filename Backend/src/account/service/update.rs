use crate::account::material::account::Account;
use crate::account::tools::update::Update;
use crate::account::material::post_change_str::PostChangeStr;
use crate::account::domainvalue::validation_pair::ValidationPair;
use crate::account::domainvalue::account_information::AccountInformation;

use rocket::State;
use rocket_contrib::json::Json;

#[openapi]
#[post("/update/password", data = "<params>")]
pub fn password(me: State<Account>, params: Json<PostChangeStr>) -> Result<Json<ValidationPair>, String>
{
  match me.change_password(&params) {
    Ok(val_pair) => Ok(Json(val_pair)),
    Err(error_str) => Err(error_str)
  }
}

#[openapi]
#[post("/update/nickname", data = "<params>")]
pub fn nickname(me: State<Account>, params: Json<PostChangeStr>) -> Result<Json<AccountInformation>, String>
{
  match me.change_name(&params) {
    Ok(acc_info) => Ok(Json(acc_info)),
    Err(error_str) => Err(error_str)
  }
}

#[openapi]
#[post("/update/mail", data = "<params>")]
pub fn mail(me: State<Account>, params: Json<PostChangeStr>) -> Result<Json<ValidationPair>, String>
{
  match me.change_mail(&params) {
    Ok(val_pair) => Ok(Json(val_pair)),
    Err(error_str) => Err(error_str)
  }
}