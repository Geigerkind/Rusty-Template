use rocket::State;
use rocket_contrib::json::Json;

use crate::account::domain_value::{AccountInformation, ValidationPair, UpdateContent, UpdateContentCredentials};
use crate::account::material::Account;
use crate::account::tools::Update;

#[post("/update/password", data = "<params>")]
pub fn password(me: State<Account>, params: Json<UpdateContent>) -> Result<Json<ValidationPair>, String>
{
  match me.change_password(&params) {
    Ok(val_pair) => Ok(Json(val_pair)),
    Err(error_str) => Err(error_str)
  }
}

#[post("/update/nickname", data = "<params>")]
pub fn nickname(me: State<Account>, params: Json<UpdateContent>) -> Result<Json<AccountInformation>, String>
{
  match me.change_name(&params) {
    Ok(acc_info) => Ok(Json(acc_info)),
    Err(error_str) => Err(error_str)
  }
}

#[post("/update/mail", data = "<params>")]
pub fn mail(me: State<Account>, params: Json<UpdateContentCredentials>) -> Result<Json<ValidationPair>, String>
{
  match me.change_mail(&params) {
    Ok(val_pair) => Ok(Json(val_pair)),
    Err(error_str) => Err(error_str)
  }
}