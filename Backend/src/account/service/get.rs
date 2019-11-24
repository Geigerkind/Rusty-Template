use rocket::State;
use rocket_contrib::json::Json;

use crate::account::domain_value::AccountInformation;
use crate::account::material::Account;
use crate::account::tools::GetAccountInformation;

#[get("/get/<id>")]
pub fn get(me: State<Account>, id: u32) -> Result<Json<AccountInformation>, String>
{
  match me.get(id) {
    Ok(acc_info) => Ok(Json(acc_info)),
    Err(err_str) => Err(err_str)
  }
}